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

procedure listarPorDato(var a: ArchivoEmpleados);
var
    e: TEmpleado;
    dato: string;
    op: integer;
begin
    Reset(a);
    writeln('Buscar por:');
    writeln('1. Apellido');
    writeln('2. Nombre');
    write('Opción: ');
    readln(op);
    write('Ingrese valor a buscar: ');
    readln(dato);
    
    while not EOF(a) do begin
        Read(a, e);
        if ((op = 1) and (e.apellido = dato)) or 
           ((op = 2) and (e.nombre = dato)) then
        begin
            writeln('Encontrado: ', e.apellido, ', ', e.nombre, 
                    ' | Nro: ', e.numero, ' | Edad: ', e.edad);
        end;
    end;
    Close(a);
end;

procedure listarTodos(var a: ArchivoEmpleados);
var
    e: TEmpleado;
begin
    Reset(a);
    while not EOF(a) do begin
        Read(a, e);
        writeln(e.apellido, ', ', e.nombre, 
               ' | Nro: ', e.numero, 
               ' | Edad: ', e.edad, 
               ' | DNI: ', e.dni);
    end;
    Close(a);
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
        writeln('4. Volver al menu principal');
        write('Opción: ');
        readln(op);
        
        case op of
            1: listarPorDato(a);
            2: listarTodos(a);
            3: listarJubilados(a);
        end;
    until op = 4;
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
