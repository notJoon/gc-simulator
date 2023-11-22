from typing import List

from object import Object

class Memory:
    def __init__(self) -> None:
        self.objects: List[Object] = []

    def add_object(self, o: Object) -> None:
        self.objects.append(o)

    def remove_object(self, o: Object) -> None:
        self.objects.remove(o)