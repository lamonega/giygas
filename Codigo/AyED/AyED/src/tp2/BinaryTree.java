package tp2;
import java.util.*;

public class BinaryTree <T> {
	
    private T data;
    private BinaryTree<T> leftChild;   
    private BinaryTree<T> rightChild; 

    public BinaryTree() {
        super();
    }

    public BinaryTree(T data) {
        this.data = data;
    }

    public T getData() {
        return data;
    }

    public void setData(T data) {
        this.data = data;
    }
    /**
     * Preguntar antes de invocar si hasLeftChild()
     * @return
     */
    public BinaryTree<T> getLeftChild() {
        return leftChild;
    }
    /**
     * Preguntar antes de invocar si hasRightChild()
     * @return
     */
    public BinaryTree<T> getRightChild() {
        return rightChild;
    }

    public void addLeftChild(BinaryTree<T> child) {
        this.leftChild = child;
    }

    public void addRightChild(BinaryTree<T> child) {
        this.rightChild = child;
    }

    public void removeLeftChild() {
        this.leftChild = null;
    }

    public void removeRightChild() {
        this.rightChild = null;
    }

    public boolean isEmpty(){
        return (this.isLeaf() && this.getData() == null);
    }

    public boolean isLeaf() {
        return (!this.hasLeftChild() && !this.hasRightChild());
    }

    public boolean hasLeftChild() {
        return this.leftChild!=null;
    }

    public boolean hasRightChild() {
        return this.rightChild!=null;
    }
    @Override
    public String toString() {
        return this.getData().toString();
    }

/* Ejercicio 2
Agregue a la clase BinaryTree los siguientes métodos:
a) contarHojas():int Devuelve la cantidad de árbol/subárbol hojas del árbol receptor.
b) espejo(): BinaryTree<T> Devuelve el árbol binario espejo del árbol receptor.
c) entreNiveles(int n, m) Imprime el recorrido por niveles de los elementos del árbol
receptor entre los niveles n y m (ambos inclusive). (0≤n<m≤altura del árbol) */    
    
    public int contarHojas() {
        int leftC =0; 
        int rightC = 0;
        if (this.isEmpty()) return 0;
        else if(this.isLeaf()) return 1;
        else {
            if(this.hasLeftChild()) 
                leftC = this.getLeftChild().contarHojas();
            if(this.hasRightChild()) 
                rightC = this.getRightChild().contarHojas();
            return leftC + rightC;
        }
    }	
    	 
    public BinaryTree<T> espejo(){
        BinaryTree<T> arbEspejo = new BinaryTree<T>(this.getData());
        if(this.hasLeftChild()) {
            arbEspejo.addRightChild(this.getLeftChild().espejo());
        }
        if(this.hasRightChild()) {
            arbEspejo.addLeftChild(this.getRightChild().espejo());
        }
        return arbEspejo;
    }
    
    public void entreNiveles(int n, int m) {
        if (this.isEmpty() || n < 0 || m < n) return; 
        Queue<BinaryTree<T>> cola = new LinkedList();
        cola.add(this);
        int nivelActual = 0;
        
        while(!cola.isEmpty()) {
            int nodoNivel = cola.size();
            if(nivelActual >=n && nivelActual <= m) {
                for(int i=0; i < nodoNivel; i++) {
                    BinaryTree<T> nodo = cola.remove();
                    System.out.print(nodo.getData() + " | ");
                    if(nodo.hasLeftChild()) cola.add(nodo.getLeftChild());
                    if(nodo.hasRightChild()) cola.add(nodo.getRightChild());
                }
            } else {
                for(int i=0; i < nodoNivel; i++) {
                    cola.remove();
                }
            }
            nivelActual++;
            System.out.println("");
        }
    }
}