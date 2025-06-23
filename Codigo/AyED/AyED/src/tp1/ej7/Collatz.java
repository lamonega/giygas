package tp1.ej7;

import java.util.LinkedList;

public class Collatz {
	public LinkedList<Integer> SucesionCollatz (Integer n) {
		LinkedList<Integer> l = new LinkedList<Integer>();
		l.add(n);
		if (n == 1) return l;
		if (n % 2 == 0) {
			l.addAll(SucesionCollatz(n /2));
		} else {
			l.addAll(SucesionCollatz(3 * n + 1));
		}
		return l;
	}
}
