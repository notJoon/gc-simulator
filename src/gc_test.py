import unittest

from controller import PIController
from mark_and_sweep import MarkAndSweepGC
from mem import Memory
from object import Object

class TestGarbageCollector(unittest.TestCase):
    def test_object_creation(self):
        obj = Object("TestObject")
        self.assertEqual(obj.ident, "TestObject")

    def test_add_reference(self):
        obj1 = Object("Object1")
        obj2 = Object("Object2")
        Obj3 = Object("Object3")
        self.assertEqual(len(obj1.references), 0)

        obj1.add_reference(obj2)
        self.assertIn(obj2, obj1.references)
        self.assertEqual(len(obj1.references), 1)

        obj1.add_reference(Obj3)
        self.assertIn(Obj3, obj1.references)
        self.assertEqual(len(obj1.references), 2)

        Obj4 = Object("Object4")
        Obj3.add_reference(Obj4)
        self.assertIn(Obj4, Obj3.references)
        self.assertEqual(len(Obj3.references), 1)
        self.assertEqual(len(obj1.references), 2)

        print(obj1.references)

    def test_remove_reference(self):
        obj1 = Object("Object1")
        obj2 = Object("Object2")
        obj1.add_reference(obj2)
        obj1.remove_reference(obj2)
        self.assertNotIn(obj2, obj1.references)

    def test_mark(self):
        memory = Memory()
        gc = MarkAndSweepGC(memory)

        root = Object("Root")
        child1 = Object("Child1")
        child2 = Object("Child2")

        root.add_reference(child1)
        child1.add_reference(child2)

        memory.add_object(root)
        memory.add_object(child1)
        memory.add_object(child2)

        gc.mark(root)

        self.assertTrue(root.marked)
        self.assertTrue(child1.marked)
        self.assertTrue(child2.marked)

    def test_mark_and_sweep(self):
        memory = Memory()
        gc = MarkAndSweepGC(memory)

        root = Object("Root")
        memory.add_object(root)
        child1 = Object("Child1")
        child2 = Object("Child2")
        root.add_reference(child1)
        memory.add_object(child1)
        memory.add_object(child2)

        gc.collect_garbage([root])

        self.assertIn(root, memory.objects)
        self.assertIn(child1, memory.objects)
        self.assertNotIn(child2, memory.objects)

if __name__ == '__main__':
    unittest.main()
