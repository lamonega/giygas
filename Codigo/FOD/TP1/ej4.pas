program ejercicio4;

type
    Empleado = record
        numero: integer;
        apellido: string[20];
        nombre: string[15];
        edad: integer;
        dni: integer;
    end;

    ArchivoEmpleados = file of Empleado;

{------------------------ UTILIDADES ------------------------}
procedure ImprimirEmpleado(emp: Empleado);
begin
    writeln('Numero=', emp.numero, ' Apellido=', emp.apellido, ' Nombre=', emp.nombre, ' Edad=', emp.edad, ' DNI=', emp.dni);
end;

procedure LeerEmpleado(var emp: Empleado);
begin
    writeln('Ingrese el apellido del empleado o "fin" para finalizar:');
    readln(emp.apellido);
    if emp.apellido <> 'fin' then
    begin
        writeln('Ingrese el nombre del empleado:');
        readln(emp.nombre);
        writeln('Ingrese el numero del empleado:');
        readln(emp.numero);
        writeln('Ingrese la edad del empleado:');
        readln(emp.edad);
        writeln('Ingrese el DNI del empleado:');
        readln(emp.dni);
    end;
end;

{------------------------ CARGA INICIAL ------------------------}
procedure CargarEmpleados(var archivo: ArchivoEmpleados);
var
    emp: Empleado;
begin
    LeerEmpleado(emp);
    while emp.apellido <> 'fin' do
    begin
        write(archivo, emp);
        LeerEmpleado(emp);
    end;
    close(archivo);
end;

{------------------------ CONSULTA POR NOMBRE/APELLIDO ------------------------}
function CoincideNombreApellido(nombre, apellido, texto: string): boolean;
begin
    CoincideNombreApellido := (nombre = texto) or (apellido = texto);
end;

procedure BuscarPorNombreApellido(var archivo: ArchivoEmpleados);
var
    texto: string[20];
    emp: Empleado;
begin
    reset(archivo);
    writeln('Ingrese un nombre o apellido a buscar:');
    readln(texto);
    writeln('Empleados encontrados:');
    while not eof(archivo) do
    begin
        read(archivo, emp);
        if CoincideNombreApellido(emp.nombre, emp.apellido, texto) then
            ImprimirEmpleado(emp);
    end;
    close(archivo);
end;

{------------------------ IMPRESIONES ------------------------}
procedure ImprimirArchivoCompleto(var archivo: ArchivoEmpleados);
var
    emp: Empleado;
begin
    reset(archivo);
    writeln('Archivo completo:');
    while not eof(archivo) do
    begin
        read(archivo, emp);
        ImprimirEmpleado(emp);
    end;
    close(archivo);
end;

procedure ListarEmpleadosJubilacion(var archivo: ArchivoEmpleados);
var
    emp: Empleado;
begin
    reset(archivo);
    writeln('Empleados mayores de 70 años:');
    while not eof(archivo) do
    begin
        read(archivo, emp);
        if emp.edad > 70 then
            ImprimirEmpleado(emp);
    end;
    close(archivo);
end;

{------------------------ INCISO A ------------------------}
function EsNumeroUnico(var archivo: ArchivoEmpleados; numero: integer): boolean;
var
    emp: Empleado;
    encontrado: boolean;
begin
    encontrado := false;
    seek(archivo, 0); // Reinicio posición de lectura
    while not eof(archivo) and not encontrado do
    begin
        read(archivo, emp);
        if emp.numero = numero then
            encontrado := true;
    end;
    EsNumeroUnico := not encontrado;
end;

procedure AñadirEmpleadosAlFinal(var archivo: ArchivoEmpleados);
var
    emp: Empleado;
begin
    reset(archivo);
    LeerEmpleado(emp);
    while emp.apellido <> 'fin' do
    begin
        if EsNumeroUnico(archivo, emp.numero) then
        begin
            seek(archivo, filesize(archivo));
            write(archivo, emp);
        end
        else
            writeln('Empleado con numero ', emp.numero, ' ya existe. No se agrega.');
        LeerEmpleado(emp);
    end;
    close(archivo);
end;

{------------------------ INCISO B ------------------------}
procedure ModificarEdadEmpleado(var archivo: ArchivoEmpleados);
var
    numeroEmpleado, nuevaEdad: integer;
    emp: Empleado;
    encontrado: boolean;
begin
    reset(archivo);
    writeln('Ingrese el numero del empleado cuya edad desea modificar:');
    readln(numeroEmpleado);
    writeln('Ingrese la nueva edad:');
    readln(nuevaEdad);
    encontrado := false;

    while not eof(archivo) and not encontrado do
    begin
        read(archivo, emp);
        if emp.numero = numeroEmpleado then
        begin
            emp.edad := nuevaEdad;
            seek(archivo, filepos(archivo) - 1);
            write(archivo, emp);
            encontrado := true;
        end;
    end;

    if encontrado then
        writeln('Edad modificada exitosamente.')
    else
        writeln('Empleado no encontrado.');
        
    close(archivo);
end;

{------------------------ INCISO C ------------------------}
procedure ExportarArchivoCompleto(var archivo: ArchivoEmpleados);
var
    archivoTxt: text;
    emp: Empleado;
begin
    assign(archivoTxt, 'todos_empleados.txt');
    reset(archivo);
    rewrite(archivoTxt);
    while not eof(archivo) do
    begin
        read(archivo, emp);
        with emp do
            writeln(archivoTxt, numero, ' ', apellido, ' ', nombre, ' ', edad, ' ', dni);
    end;
    close(archivo);
    close(archivoTxt);
    writeln('Archivo exportado como "todos_empleados.txt".');
end;

{------------------------ INCISO D ------------------------}
procedure ExportarEmpleadosSinDNI(var archivo: ArchivoEmpleados);
var
    archivoTxt: text;
    emp: Empleado;
begin
    assign(archivoTxt, 'faltaDNIEmpleado.txt');
    reset(archivo);
    rewrite(archivoTxt);
    while not eof(archivo) do
    begin
        read(archivo, emp);
        if emp.dni = 0 then
            with emp do
                writeln(archivoTxt, numero, ' ', apellido, ' ', nombre, ' ', edad, ' ', dni);
    end;
    close(archivo);
    close(archivoTxt);
    writeln('Archivo exportado como "faltaDNIEmpleado.txt".');
end;

{------------------------ MENÚ ------------------------}
procedure MostrarMenu();
begin
    writeln;
    writeln('===== MENU DE OPCIONES =====');
    writeln('1. Buscar empleado por nombre o apellido');
    writeln('2. Mostrar todos los empleados');
    writeln('3. Listar empleados mayores de 70 años');
    writeln('4. Añadir nuevos empleados');
    writeln('5. Modificar edad de un empleado');
    writeln('6. Exportar empleados a "todos_empleados.txt"');
    writeln('7. Exportar empleados sin DNI a "faltaDNIEmpleado.txt"');
    writeln('8. Salir');
    writeln('Seleccione una opción:');
end;

procedure MenuOpciones(var archivo: ArchivoEmpleados);
var
    opcion: integer;
begin
    MostrarMenu;
    readln(opcion);
    while opcion <> 8 do
    begin
        case opcion of
            1: BuscarPorNombreApellido(archivo);
            2: ImprimirArchivoCompleto(archivo);
            3: ListarEmpleadosJubilacion(archivo);
            4: AñadirEmpleadosAlFinal(archivo);
            5: ModificarEdadEmpleado(archivo);
            6: ExportarArchivoCompleto(archivo);
            7: ExportarEmpleadosSinDNI(archivo);
        else
            writeln('Opción inválida.');
        end;
        MostrarMenu;
        readln(opcion);
    end;
end;

{------------------------ PROGRAMA PRINCIPAL ------------------------}
var
    archivo: ArchivoEmpleados;
    nombreArchivo: string[15];
begin
    writeln('Ingrese el nombre del archivo:');
    readln(nombreArchivo);
    assign(archivo, nombreArchivo);
    rewrite(archivo);
    CargarEmpleados(archivo);
    MenuOpciones(archivo);
end.
