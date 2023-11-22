from typing import List

from mem import Memory
from object import Object

class MarkAndSweepGC:
    def __init__(self, m: Memory) -> None:
        self.mem: Memory = m

    def mark_all(self, roots: List[Object]) -> None:
        for obj in roots:
            self.mark(obj)

    def mark(self, obj: Object) -> None:
        if not obj.marked:
            obj.marked = True
            for ref in obj.references:
                self.mark(ref)

    def sweep(self) -> None:
        self.mem.objects = [obj for obj in self.mem.objects if obj.marked]
        for obj in self.mem.objects:
            obj.marked = False

    def collect_garbage(self, roots: List[Object]) -> str:
        if len(roots) == 1:
            self.mark(roots[0])
        else:
            self.mark_all(roots)
        self.sweep()

        return "Garbage collected"