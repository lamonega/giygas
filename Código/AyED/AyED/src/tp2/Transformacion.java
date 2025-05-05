package tp2;

public class Transformacion {
    private BinaryTree<Integer> ab;
    
    public Transformacion(BinaryTree<Integer> unArbol) {
        ab = unArbol;
    }

    public BinaryTree<Integer> getAb() {
        return ab;
    }
    
    public BinaryTree <Integer> suma() {
        suma(ab);
        return ab;
    }
    
    private int suma(BinaryTree<Integer>ab) {
        int sum = 0;
        if(ab.isLeaf()) {
            sum = ab.getData();
            ab.setData(0);
            return sum;
        }
        if(ab.hasLeftChild()) {
            sum+= suma(ab.getLeftChild());
        }
        if(ab.hasRightChild()) {
            sum+= suma(ab.getRightChild());
        }
        int actual = ab.getData();
        ab.setData(sum);
        return actual + sum;
    }
}