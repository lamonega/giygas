// (7f)
package tp1.ej7;

import java.util.List;

public class Capicúa {
    public static boolean esCapicua(List<Integer> l) {
        int inicio = 0;
        int fin = l.size() - 1;
        while (inicio < fin) {
            if (!l.get(inicio).equals(l.get(fin))) {
                return false;
            }
            inicio++;
            fin--;
        }
        return true;
    }
}