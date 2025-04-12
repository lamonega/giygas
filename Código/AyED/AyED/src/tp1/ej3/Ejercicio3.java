package tp1.ej3;

public class Ejercicio3 {
	public static void main(String[] args) {
        Estudiante[] estudiantes = new Estudiante[2];
        estudiantes[0] = new Estudiante();
        estudiantes[0].setNombre("Ana");
        estudiantes[0].setApellido("Pérez");
        //.. otros setters

        Profesor[] profesores = new Profesor[3];
        profesores[0] = new Profesor();
        profesores[0].setNombre("Carlos");
        // ... otros setters

        for (Estudiante e : estudiantes) {
            System.out.println(e.tusDatos()); // Breakpoint aquí
        }

        for (Profesor p : profesores) {
            System.out.println(p.tusDatos()); // Breakpoint aquí
        }
    }
}
