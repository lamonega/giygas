package tp1.ej5;

public class Ejercicio5 {
	// a. Retornando con return
    public static double[] calcularConReturn(int[] arr) {
        int max = arr[0], min = arr[0], sum = 0;
        for (int num : arr) { // Para cada número del arreglo
            max = Math.max(max, num); // Veo si es máximo
            min = Math.min(min, num); // Veo si es mínimo
            sum += num; // Lo acumulo
        }
        return new double[]{max, min, (double)sum/arr.length}; // Retorno
        // Un arreglo de dobles que contendrá max casteado a doble
        // min casteado a doble
        // sum dividido entre el largo del arreglo downcasteado a doble
    }

    // b. Usando parámetros (clase auxiliar)
    public static void calcularConParametros(int[] arr, Resultado res) {
    	// Uso la clase auxiliar Res para devolver el resultado
    	// Aunque hay un parámetro arreglo, es simplemente el arreglo que recibo
    	// Sino, como voy a operar?
    	
    	// Mismo procedimiento que antes
        res.max = arr[0];
        res.min = arr[0];
        int sum = 0;
        for (int num : arr) {
            res.max = Math.max(res.max, num);
            res.min = Math.min(res.min, num); // Solo que sobreescribo directamente los datos de Res
            sum += num;
        }
        res.promedio = (double)sum / arr.length; // Lo mismo, sobreescribo directamente res.
    }

    // c. Usando variables estáticas
    public static class Stats {
        public static int max, min;
        public static double promedio;
    }

    public static void calcularSinReturn(int[] arr) {
        Stats.max = arr[0];
        Stats.min = arr[0];
        int sum = 0;
        for (int num : arr) {
            Stats.max = Math.max(Stats.max, num);
            Stats.min = Math.min(Stats.min, num);
            sum += num;
        }
        Stats.promedio = (double)sum / arr.length; // Nada muy loco
    }

    public static class Resultado {
        int max, min;
        double promedio;
    }
}