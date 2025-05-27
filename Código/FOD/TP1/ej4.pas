program ejercicio3;
type
    TEmpleado = record
        numero: integer;
        edad: integer;
        dni: string[10];
        apellido: string[50];
        nombre: string[50];
    end;

    ArchivoEmpleados = file of TEmpleado;

// Procedimientos y funciones

procedure imprimirEmpleado(e: empleado);
begin
    writeln('Numero=', e.numero, ' Apellido=', e.apellido, ' Nombre=', e.nombre, ' Edad=', e.edad, ' DNI=', e.dni);
end;

procedure leerEmpleado(var e: TEmpleado);
begin
    write('Apellido ("fin" para terminar): ');
    readln(e.apellido);
    if e.apellido <> 'fin' then begin
        write('Nombre: ');
        readln(e.nombre);
        write('Número: ');
        readln(e.numero);
        write('Edad: ');
        readln(e.edad);
        write('DNI (00 si no tiene): ');
        readln(e.dni);
    end;
end;

procedure crearArchivo(var a: ArchivoEmpleados);
var
    empleado: TEmpleado;
    nombreFisico: string;
begin
    write('Ingrese nombre del archivo: ');
    readln(nombreFisico);
    Assign(a, nombreFisico);
    Rewrite(a);  // Crear nuevo archivo
    
    leerEmpleado(empleado);
    while empleado.apellido <> 'fin' do begin
        Write(a, empleado);
        leerEmpleado(empleado);
    end;
    
    Close(a);
    writeln('Archivo creado exitosamente!');
end;

function cumple(n, a, t: string): boolean;
begin
    cumple:= ((n = t) or (a = t));
end;
procedure empleadoApellONombre(var a: ArchivoEmpleados);
var
    s: string[20];
    e: TEmpleado;
begin
    reset(a);
    writeln('Ingrese un nombre o un apellido de un empleado');
    readln(s);
    writeln('Empleados que tienen un nombre o apellido iguales a ', s, ': ');
    while(not EOF(a)) do
        begin
            read(a, e);
            if(cumple(e.nombre, e.apellido, s)) then
                imprimirEmpleado(e);
        end;
    close(a);
end;

procedure imprimirArchivo(var a: ArchivoEmpleados);
var
    e: TEmpleado;
begin
    reset(a);
    write('Archivo completo: ');
    while(not EOF(arc)) do
        begin
            read(a, e);
            imprimirEmpleado(e);
        end;
    close(a);
end;

procedure listarJubilados(var a: ArchivoEmpleados);
var
    e: TEmpleado;
begin
    Reset(a);
    while not EOF(a) do begin
        Read(a, e);
        if e.edad > 70 then begin
            writeln('[Jubilación] ', e.apellido, ', ', e.nombre, 
                    ' | Edad: ', e.edad);
        end;
    end;
    Close(a);
end;

procedure añadirAlFinal(var a: ArchivoEmpleados);
var
    e: TEmpleado;
begin
    Reset(a);
end;

procedure exportarTodo();
begin

end;

procedure exportarEmpleadosSinDNI();
begin

end;

procedure abrirArchivo(var a: ArchivoEmpleados);
var
    op: integer;
    nombreFisico: string;
begin
    write('Ingrese nombre del archivo: ');
    readln(nombreFisico);
    Assign(a, nombreFisico);
    
    repeat
        writeln('--- MENU ARCHIVO ---');
        writeln('1. Buscar por nombre/apellido');
        writeln('2. Listar todos');
        writeln('3. Listar próximos a jubilarse');
        writeln('4. Añadir empleado/s al final');
        writeln('5. Modificar la edad de un empleado');
        writeln('6. Exportar lista de empleados a .txt');
        writeln('7. Exportar lista de empleados sin DNI a .txt');
        writeln('8. Volver al menu principal');
        write('Opción: ');
        readln(op);
        
        case op of
            1: listarPorDato(a);
            2: listarTodos(a);
            3: listarJubilados(a);
        end;
    until op = 8;
end;

// Programa principal
var
    archivo: ArchivoEmpleados;
    opcion: integer;
begin
    repeat
        writeln('----- MENU PRINCIPAL -----');
        writeln('1. Crear nuevo archivo');
        writeln('2. Abrir archivo existente');
        writeln('3. Salir');
        write('Seleccione opción: ');
        readln(opcion);
        
        case opcion of
            1: crearArchivo(archivo);
            2: abrirArchivo(archivo);
        end;
    until opcion = 3;
end.