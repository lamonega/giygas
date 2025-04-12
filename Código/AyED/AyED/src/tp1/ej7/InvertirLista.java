package tp1.ej7;

import java.util.ArrayList;

public class InvertirLista {
	public void Inversión (ArrayList<Integer> lista) {
		if (lista.size() > 1) {
		int pri = lista.remove(0);
		int ult = lista.remove(lista.size() - 1);
		Inversión(lista);
		
		lista.add(0, ult);
		lista.add(pri);
		}
	}
}
