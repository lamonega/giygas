package tp1.ej5;

public class Ejercicio5 {
    public static void main(String[] args) {
        int[] arreglo1 = {10, 51, 30, 42, 50};
        int[] arreglo2 = {12, 28, 30, 38, 20};
        Datos d = new Datos();
        int[] arreglo3 = {5, 20, 11, 40, 100};
        
        System.out.println(Calculadora.maxminpromA(arreglo1).toString());
        Calculadora.maxminpromB(arreglo2, d);
        System.out.println(d.toString());
        Calculadora.maxminpromC(arreglo3);
        System.out.println(Calculadora.obtenerResultados());
    }
    
}