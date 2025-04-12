// (7d)
package tp1.ej7;

import java.util.LinkedList;

public class ListasEstudiantes {
    public static class Estudiante {
        private String nombre;
        private String legajo;
        
        public Estudiante(String nombre, String legajo) {
            this.nombre = nombre;
            this.legajo = legajo;
        }
        
        public String getNombre() { return nombre; }
        public void setNombre(String nombre) { this.nombre = nombre; }
        public String getLegajo() { return legajo; }
        public void setLegajo(String legajo) { this.legajo = legajo; }
        
        @Override
        public String toString() {
            return "Estudiante [nombre=" + nombre + ", legajo=" + legajo + "]";
        }
        
        // Método agregarEstudiante
        public static boolean agregarEstudiante(LinkedList<ListasEstudiantes.Estudiante> l, ListasEstudiantes.Estudiante e) {
            boolean agregue = false;
            if (!l.contains(e)) {
                l.add(e);
                agregue = true;
            }
            return agregue;
        }
    }
    
    @SuppressWarnings("unchecked")
	public static void main(String[] args) {
        LinkedList<Estudiante> listaEstudiantes = new LinkedList<>();
        listaEstudiantes.add(new Estudiante("Ana", "23557(3"));
        listaEstudiantes.add(new Estudiante("Benicio", "21374/1"));
        listaEstudiantes.add(new Estudiante("Carolina", "18222/2"));
        
        LinkedList<Estudiante> shallow = (LinkedList<Estudiante>) listaEstudiantes.clone();
        LinkedList<Estudiante> deep = new LinkedList<>();
        for (Estudiante e : listaEstudiantes) {
            deep.add(e);
        }
        
        System.out.println(listaEstudiantes);
        System.out.println(shallow);
        System.out.println(deep);
        
        listaEstudiantes.get(1).setNombre("Patricio");
        shallow.get(0).setNombre("Ana Banana");
        deep.get(2).setNombre("Saturnina");
        
        System.out.println(listaEstudiantes);
        System.out.println(shallow);
        System.out.println(deep);
    }
}