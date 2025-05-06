package tp1.ej8;

public class DoubleEndedQueue <T> extends Queue<T> {
    public void enqueueFirst(T data) {
        super.elements.add(0, data);
    }
}