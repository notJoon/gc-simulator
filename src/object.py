from typing import List

class Object:
    def __init__(self, ident: str) -> None:
        self.ident = ident
        self.references: List['Object'] = []
        self.marked: bool = False

    def add_reference(self, obj: 'Object') -> None:
        if obj not in self.references:
            self.references.append(obj)

    def remove_reference(self, obj: 'Object') -> None:
        if obj in self.references:
            self.references.remove(obj)

    def __repr__(self) -> str:
        references_str = [ref.ident for ref in self.references]
        return f"Object({self.ident})"
