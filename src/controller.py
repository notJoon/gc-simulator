from typing import List, TypeVar

T = TypeVar('T')

class PIController:
    def __init__(self, gc: T, threshold: int) -> None:
        self.gc: T = gc
        self.threshold = threshold

    def check_gc(self, roots: List[any]) -> str:
        if len(self.gc.mem.objects) > self.threshold:
            self.gc.collect_garbage(roots)
            return "Garbage collected"
        return "GC not needed"