// Ejercicio 1 y 2

// Ejercicio 1

package tp2;

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
		return this.rightChild;
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

// Ejercicio 2

	public  int contarHojas() {
        if (this.isLeaf()) {
            return 1;
        }

        int cantidad = 0;

        if (this.hasLeftChild()) {
            cantidad += this.getLeftChild().contarHojas();
        }
        if (this.hasRightChild()) {
            cantidad += this.getRightChild().contarHojas();
        }

        return cantidad;
	}
		
    public BinaryTree<T> espejo() {
    if (this == null || this.getData() == null) {
        return null;
    }
    BinaryTree<T> nuevo = new BinaryTree<T>(this.getData());
    if (this.hasRightChild()) {
        nuevo.addLeftChild(this.getRightChild().espejo());
    }
    if (this.hasLeftChild()) {
        nuevo.addRightChild(this.getLeftChild().espejo());
    }
    return nuevo;
}

    public void entreNiveles(int nivelInicial, int nivelFinal) {
        if (nivelInicial < 0 || nivelFinal < nivelInicial) {
            return;
        }

        Queue<BinaryTree<T>> colaNodos = new Queue<>();
        colaNodos.enqueue(this);

        int nivelActual             = 0;
        int nodosRestantesNivel     = 1;  // cuántos nodos quedan por procesar en el nivelActual
        int nodosParaNivelSiguiente = 0;  // cuántos nodos se van a procesar en el nivelActual+1

        // Mientras queden nodos en la cola y no hayamos superado nivelFinal
        while (!colaNodos.isEmpty() && nivelActual <= nivelFinal) {
            // desencolamos un nodo de este nivel
            BinaryTree<T> arbol = colaNodos.dequeue();
            nodosRestantesNivel--;

            // solo imprimimos si estamos dentro del rango pedido
            if (nivelActual >= nivelInicial) {
                System.out.print(arbol.getData() + " ");
            }

            // encolamos hijos y contamos para el siguiente nivel
            if (arbol.hasLeftChild()) {
                colaNodos.enqueue(arbol.getLeftChild());
                nodosParaNivelSiguiente++;
            }
            if (arbol.hasRightChild()) {
                colaNodos.enqueue(arbol.getRightChild());
                nodosParaNivelSiguiente++;
            }

            // si ya procesamos todos los nodos de este nivel…
            if (nodosRestantesNivel == 0) {
                // si imprimimos algo en este nivel, hacemos salto de línea
                if (nivelActual >= nivelInicial) {
                    System.out.println();
                }
                // avanzamos de nivel
                nivelActual++;
                // preparamos la cuenta para el siguiente nivel
                nodosRestantesNivel     = nodosParaNivelSiguiente;
                nodosParaNivelSiguiente = 0;
                // al volver a la cabecera, la condición nivelActual <= nivelFinal
                // decidirá si el bucle continúa o termina
            }
        }
    }
		
}
