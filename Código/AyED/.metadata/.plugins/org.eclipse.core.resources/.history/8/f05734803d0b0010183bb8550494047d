package tp1.ej8;

import java.util.LinkedList;

// 8a. Clase Queue
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

// 8b. Cola Circular
class CircularQueue<T> extends Queue<T> {
    public T shift() {
        if (isEmpty()) {
            throw new IllegalStateException("Cola vacía");
        }
        T elemento = dequeue();
        enqueue(elemento);
        return head();
    }
}

// 8c. Cola Doble
class DoubleEndedQueue<T> extends Queue<T> {
    public void enqueueFirst(T elemento) {
        if (elements instanceof LinkedList<T>) {
            ((LinkedList<T>) elements).addFirst(elemento);
        } else {
            throw new UnsupportedOperationException("Operación no soportada");
        }
    }
}