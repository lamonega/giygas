Program Restest;

{$mode delphi}
{$h+}

uses gettext;

resourcestring

  Testing = 'Testing :';
  First = 'First';
  Second = 'Second';
  Third = 'Third';
  
begin 
  { Tell gettext to translate the strings 
    according to the settings in the LANG environment variable 
    remark that the program must be run in the tests directory
    where the intl subdirectory exists }
  TranslateResourcestrings('intl/restest.%s.mo');
  Writeln(Testing);
  Writeln(First);
  Writeln(Second);
  Writeln(Third);
end.  $Log: restest.pp,v $
end.  Revision 1.2  2000/07/13 11:33:04  michael
end.  + removed logs
end. 
}
