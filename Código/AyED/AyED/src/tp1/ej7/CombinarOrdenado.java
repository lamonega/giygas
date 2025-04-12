package tp1.ej7;

import java.util.ArrayList;

public class CombinarOrdenado {
	public ArrayList<Integer> MergeSort(ArrayList<Integer> lista1, ArrayList<Integer> lista2) {
	    ArrayList<Integer> combinada = new ArrayList<>(lista1.size() + lista2.size());
	    int i = 0, j = 0;
	    while (i < lista1.size() && j < lista2.size()) {
	        if (lista1.get(i) <= lista2.get(j)) {
	            combinada.add(lista1.get(i++));
	        } else {
	            combinada.add(lista2.get(j++));
	        }
	    }
	    while (i < lista1.size()) combinada.add(lista1.get(i++));
	    while (j < lista2.size()) combinada.add(lista2.get(j++));
	    return combinada;
	}
}
