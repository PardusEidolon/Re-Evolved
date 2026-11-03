"""A reader of a WebAssembly module's imports and exports (game.wasm,
tools/engine_wasm.py).

The module is read as untrusted input: every length and count is checked
against the bytes there are before it is used, and anything malformed is
refused with MalformedModule, never read past or guessed at. Only the
sections the import and export lists need are interpreted (types, imports,
functions, exports, and the counts of tables, memories, globals and tags);
the others are skipped by their lengths.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Tuple

MAGIC = b"\x00asm"
VERSION = b"\x01\x00\x00\x00"

TYPE, IMPORT, FUNCTION, TABLE, MEMORY, GLOBAL, EXPORT = 1, 2, 3, 4, 5, 6, 7
TAG = 13

VALUE_TYPES = {
    0x7F: "i32",
    0x7E: "i64",
    0x7D: "f32",
    0x7C: "f64",
    0x7B: "v128",
    0x70: "funcref",
    0x6F: "externref",
    0x69: "exnref",
}
REFERENCE_TYPES = {0x70, 0x6F, 0x69}

KINDS = {0: "func", 1: "table", 2: "memory", 3: "global", 4: "tag"}


class MalformedModule(ValueError):
    """the bytes are not a module this reader accepts"""


@dataclass
class Module:
    """a module's imports and exports, each "name: type" (an import's name
    is "module.field"), in the module's order"""

    imports: List[str] = field(default_factory=list)
    exports: List[str] = field(default_factory=list)


class Reader:
    """bytes read forwards, with every read checked against the end"""

    def __init__(self, data: bytes, start: int = 0, end: int = -1) -> None:
        self.data = data
        self.pos = start
        self.end = len(data) if end < 0 else end

    def remaining(self) -> int:
        return self.end - self.pos

    def byte(self) -> int:
        if self.pos >= self.end:
            raise MalformedModule(f"the module ends at byte {self.pos}")
        value = self.data[self.pos]
        self.pos += 1
        return value

    def bytes(self, count: int) -> bytes:
        if count > self.remaining():
            raise MalformedModule(f"{count} bytes at {self.pos}, past the end at {self.end}")
        value = self.data[self.pos : self.pos + count]
        self.pos += count
        return value

    def u32(self) -> int:
        """an unsigned LEB128 of at most 32 bits (five bytes, the last with
        no more than four bits)"""
        value = 0
        for index in range(5):
            byte = self.byte()
            if index == 4 and byte & 0xF0:
                raise MalformedModule(f"an integer longer than 32 bits at {self.pos - 1}")
            value |= (byte & 0x7F) << (7 * index)
            if not byte & 0x80:
                return value
        raise MalformedModule(f"an integer longer than 32 bits at {self.pos - 1}")

    def count(self) -> int:
        """a vector's length, which cannot exceed the bytes left (every
        entry takes at least one)"""
        value = self.u32()
        if value > self.remaining():
            raise MalformedModule(f"{value} entries at {self.pos}, but {self.remaining()} bytes")
        return value

    def name(self) -> str:
        raw = self.bytes(self.u32())
        try:
            return raw.decode("utf-8")
        except UnicodeDecodeError as error:
            raise MalformedModule(f"a name that is not UTF-8 at {self.pos - len(raw)}") from error

    def value_type(self) -> str:
        code = self.byte()
        if code not in VALUE_TYPES:
            raise MalformedModule(f"value type {code:#x} at {self.pos - 1}")
        return VALUE_TYPES[code]

    def reference_type(self) -> str:
        code = self.byte()
        if code not in REFERENCE_TYPES:
            raise MalformedModule(f"reference type {code:#x} at {self.pos - 1}")
        return VALUE_TYPES[code]

    def limits(self) -> None:
        """a table's or a memory's limits (bit 0 of the flags: a maximum
        follows; bit 1: shared; bit 2: 64-bit), checked and dropped"""
        flags = self.byte()
        if flags & ~0x07:
            raise MalformedModule(f"limits flags {flags:#x} at {self.pos - 1}")
        read = self.u64 if flags & 0x04 else self.u32
        read()
        if flags & 0x01:
            read()

    def u64(self) -> int:
        value = 0
        for index in range(10):
            byte = self.byte()
            if index == 9 and byte & 0xFE:
                raise MalformedModule(f"an integer longer than 64 bits at {self.pos - 1}")
            value |= (byte & 0x7F) << (7 * index)
            if not byte & 0x80:
                return value
        raise MalformedModule(f"an integer longer than 64 bits at {self.pos - 1}")

    def done(self, what: str) -> None:
        if self.pos != self.end:
            raise MalformedModule(f"the {what} section has {self.remaining()} bytes left over")


def signature(params: Tuple[str, ...], results: Tuple[str, ...]) -> str:
    return f"({', '.join(params)}) -> ({', '.join(results)})"


def sections(data: bytes) -> Dict[int, Reader]:
    """the module's sections by id, each a reader of its own bytes; custom
    sections are skipped, and any other may appear once"""
    if len(data) < 8 or data[:4] != MAGIC:
        raise MalformedModule("not a WebAssembly module")
    if data[4:8] != VERSION:
        raise MalformedModule(f"version {data[4:8].hex()}, not 1")
    reader = Reader(data, 8)
    found: Dict[int, Reader] = {}
    while reader.remaining():
        ident = reader.byte()
        size = reader.u32()
        if size > reader.remaining():
            raise MalformedModule(f"section {ident} of {size} bytes, past the end")
        body = Reader(data, reader.pos, reader.pos + size)
        reader.pos += size
        if ident == 0:
            continue
        if ident in found:
            raise MalformedModule(f"a second section {ident}")
        found[ident] = body
    return found


def read_module(data: bytes) -> Module:
    """the imports and exports of the module in data; MalformedModule if it
    is not well formed as far as they need"""
    found = sections(data)
    module = Module()

    types: List[str] = []
    if TYPE in found:
        reader = found[TYPE]
        for _ in range(reader.count()):
            form = reader.byte()
            if form != 0x60:
                raise MalformedModule(f"type form {form:#x} at {reader.pos - 1}")
            params = tuple(reader.value_type() for _ in range(reader.count()))
            results = tuple(reader.value_type() for _ in range(reader.count()))
            types.append(signature(params, results))
        reader.done("type")

    def function_type(reader: Reader) -> str:
        index = reader.u32()
        if index >= len(types):
            raise MalformedModule(f"type {index} of {len(types)}")
        return types[index]

    # the index spaces: imports first, then the module's own
    functions: List[str] = []
    counts = {kind: 0 for kind in KINDS.values()}
    if IMPORT in found:
        reader = found[IMPORT]
        for _ in range(reader.count()):
            where = f"{reader.name()}.{reader.name()}"
            kind = reader.byte()
            if kind == 0:
                described = function_type(reader)
                functions.append(described)
                module.imports.append(f"{where}: func {described}")
            elif kind == 1:
                element = reader.reference_type()
                reader.limits()
                module.imports.append(f"{where}: table {element}")
            elif kind == 2:
                reader.limits()
                module.imports.append(f"{where}: memory")
            elif kind == 3:
                value = reader.value_type()
                mutable = reader.byte()
                if mutable > 1:
                    raise MalformedModule(f"global mutability {mutable} at {reader.pos - 1}")
                module.imports.append(f"{where}: global {'mut ' if mutable else ''}{value}")
            elif kind == 4:
                if reader.byte() != 0:
                    raise MalformedModule(f"tag attribute at {reader.pos - 1}")
                module.imports.append(f"{where}: tag {function_type(reader)}")
            else:
                raise MalformedModule(f"import kind {kind} at {reader.pos - 1}")
            counts[KINDS[kind]] += 1
        reader.done("import")

    if FUNCTION in found:
        reader = found[FUNCTION]
        for _ in range(reader.count()):
            functions.append(function_type(reader))
        reader.done("function")
    counts["func"] = len(functions)
    for kind, ident in (("table", TABLE), ("memory", MEMORY), ("global", GLOBAL), ("tag", TAG)):
        if ident in found:
            counts[kind] += found[ident].count()

    if EXPORT in found:
        reader = found[EXPORT]
        for _ in range(reader.count()):
            exported = reader.name()
            kind = reader.byte()
            if kind not in KINDS:
                raise MalformedModule(f"export kind {kind} at {reader.pos - 1}")
            index = reader.u32()
            if index >= counts[KINDS[kind]]:
                raise MalformedModule(f"export of {KINDS[kind]} {index} of {counts[KINDS[kind]]}")
            if kind == 0:
                module.exports.append(f"{exported}: func {functions[index]}")
            else:
                module.exports.append(f"{exported}: {KINDS[kind]}")
        reader.done("export")
    return module
