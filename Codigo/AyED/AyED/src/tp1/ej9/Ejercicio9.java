package tp1.ej9;

import java.util.*;

public class Ejercicio9 {

    private static boolean esBalanceado(String expr) {
        List<Character> cierre = new LinkedList<Character>();
        cierre.add(')');
        cierre.add(']');
        cierre.add('}');
        boolean ok = true;
        
        if((expr.length() %2 !=0) || (expr.length() > 0 && cierre.contains(expr.charAt(0)))) ok = false; //(1)
        else {
            List<Character> apertura = new LinkedList<Character>();
            apertura.add('(');
            apertura.add('[');
            apertura.add('{');
            
            Stack<Character> pila = new Stack<Character>();
            Character actual, elem;
            int i = 0;
            while(i < expr.length() && ok) {
                actual = expr.charAt(i);
                if(apertura.contains(actual))
                    pila.push(actual);
                else {
                    if(!pila.isEmpty()) {
                        elem = pila.pop();
                        if(apertura.indexOf(elem) != cierre.indexOf(actual)) ok = false; //(2)
                    }
                }
                i++;
            }
            if(!pila.isEmpty()) ok = false; //(3)
        }
        return ok;
    }

    public static void main(String[] args) {
        Scanner consola = new Scanner(System.in);
        System.out.println("Ingrese un String");
        String s = consola.nextLine();
        consola.close(); //No se va a leer mas datos de teclado. Cierro el objeto Scanner, libero el recurso.
        if(esBalanceado(s))
            System.out.println("La expresion " + s + " esta balanceada");
        else System.out.println("La expresion " + s + " no esta balanceada");
    }
}