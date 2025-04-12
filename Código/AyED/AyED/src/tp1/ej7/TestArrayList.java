package tp1.ej7;
import java.util.ArrayList;

public class TestArrayList {
	public static void main (String[] args ) {
		// 7a. agregar args a una arraylist y luego imprimirla
		ArrayList<Integer> argumentos = new ArrayList<>();
		
		for (String arg : args) {
			// itero sobre args usando un foreach
			// y a la vez convierto a int para agregar a arraylist usando add()
			// que concatena al final
			argumentos.add(Integer.parseInt(arg));
		}
		
		for (Integer a : argumentos) {
			// imprimo los numeros
			System.out.println(a);
		}
	}
}
