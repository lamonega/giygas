program ejercicio2;
var
	archivo: file of integer;
	nombre: string;
	numero, suma, menores: integer;
	promedio: real;
begin
	write('Ingrese el nombre del archivo: ');
	readln(nombre);
	Assign(archivo, nombre);
	Reset(archivo);
	suma:= 0;
	menores:= 0;
	while (not EOF) do begin
		Read(archivo, numero);
		write(numero, ' ');
		suma:= suma + numero;
		if (numero < 1500) then
			menores:= menores + 1;
	end;
	promedio:= suma / FileSize(archivo);
	write(menores);
	write(promedio);
	Close(archivo);
end.
