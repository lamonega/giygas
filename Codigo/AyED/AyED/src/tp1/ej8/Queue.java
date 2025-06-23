package tp1.ej8;

import java.util.LinkedList;

public class Queue<T> {
    protected LinkedList<T> elements;

    public Queue() {
        this.elements = new LinkedList<>();
    }

    public void enqueue(T dato) {
        elements.addLast(dato);
    }

    public T dequeue() {
        if (isEmpty()) {
            throw new IllegalStateException("Cola vacía");
        }
        return elements.removeFirst();
    }

    public T head() {
        if (isEmpty()) {
            throw new IllegalStateException("Cola vacía");
        }
        return elements.getFirst();
    }

    public boolean isEmpty() {
        return elements.isEmpty();
    }

    public int size() {
        return elements.size();
    }

    @Override
    public String toString() {
        return elements.toString();
    }
}