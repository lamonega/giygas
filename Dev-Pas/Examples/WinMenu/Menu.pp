{   $Id: menu.pp,v 1.7 2000/01/21 00:44:51 peter Exp $
  Copyright (c) 1999 by Michael van Canneyt

  Win32 menu creation example.
}

program MenuDemo;
{$MODE DELPHI}

Uses Strings,Windows;

Const AppName = 'MenuDemo';

Var AMessage: Msg;
    hWindow,hStatus,Hedit: HWnd;
    WindowClass: WndClass;
    Menu: hMenu;

Const
  EditMenus: Array[201..203] Of pchar = ('Cut','copy','paste');

Type
  TFileName = Array[0..Max_Path] Of Char;

Function SelectFile(Var FName:TFileName; Open:Boolean): Boolean;

Const
  Filter: PChar = 'Text files (*.txt)'#0'*.txt'#0'All files (*.*)'#0'*.*'#0;
  Ext: PChar = 'txt';

Var
  NameRec: OpenFileName;
Begin
  FillChar(NameRec,SizeOf(NameRec),0);
  FName[0] := #0;
  With NameRec Do
    Begin
      LStructSize := SizeOf(NameRec);
      HWndOwner := HWindow;
      LpStrFilter := Filter;
      LpStrFile := @FName;
      NMaxFile := Max_Path;
      Flags := OFN_Explorer Or OFN_HideReadOnly;
      If Open Then
        Begin
          Flags := Flags Or OFN_FileMustExist;
        End;
      LpStrDefExt := Ext;
    End;
  If Open Then
      SelectFile := GetOpenFileName(@NameRec)
  Else
      SelectFile := GetSaveFileName(@NameRec);
End;

Procedure SaveText;

Var Len: Longint;
    P: PChar;
    F: File;
    FName: TFileName;
Begin
  If SelectFile(FName,False) Then
    Begin
      Assign(F,@FName);
      Rewrite(F,1);
      Len := GetWindowTextLength(HEdit);
      GetMem(P,Len+1);
      P[Len] := #0;
      If Len>0 Then
        Begin
          GetWindowText(HEdit,P,Len+1);
          BlockWrite(F,P^,Len);
        End;
      Close(F);
      FreeMem(P,Len+1);
    End;
End;

Procedure AskSave;
Begin
  If MessageBox(HWindow,'Save text?','Edited',MB_IconQuestion Or MB_YesNo)=IdYes Then
    Begin
      SaveText;
    End;
End;

Procedure LoadText;

Var
  FName: TFileName;
  F: File;
  Len: LongInt;
  P: PChar;
Begin
  AskSave;
  If SelectFile(FName,True) Then
    Begin
      Assign(F,@FName);
      Reset(F,1);
      Len := FileSize(F);
      GetMem(P,Len+1);
      P[Len] := #0;
      If Len>0 Then BlockRead(F,P^,Len);
      Close(F);
      SetWindowText(HEdit,P);
      FreeMem(P,Len+1);
    End;
End;

Procedure NewText;

Const
  Empty: PChar = '';
Begin
  AskSave;
  SendMessage(HEdit,WM_SetText,1,LongInt(Empty));
End;

Function WindowProc (Window:HWnd;AMessage,WParam,LParam:Longint): Longint;
stdcall;
export;

Var ps: paintstruct;
    r: rect;
    StatH: Word;
    nrmenu : longint;
Begin
  WindowProc := 0;
  Case AMessage Of
    wm_Paint:
              Begin
                BeginPaint(Window,@ps);
                GetClientRect(Window,@r);
                EndPaint(Window,ps);
                Exit;
              End;
    wm_Close:
              Begin
                AskSave;
              End;
    wm_Destroy:
                Begin
                  PostQuitMessage (0);
                  Exit;
                End;
    wm_Size:
             Begin
               GetClientRect(HStatus,@R);
               StatH := R.Bottom-R.Top;
               GetClientRect(Window,@R);
               MoveWindow (hStatus,r.left,r.bottom-StatH,r.right,r.bottom,true);
               MoveWindow (HEdit,0,0,r.right-r.left,r.bottom-r.top-StatH,true);
             End;
    wm_Command:
                Begin
                  NrMenu := WParam And $FFFF;
                  Case NrMenu Of
                    101 : NewText;
                    102 : LoadText;
                    103 : SaveText;
                    104 : PostMessage(Window,WM_Close,0,0);
                    201..203: MessageBox(Window,EditMenus[NrMenu],
                                         'Edit operation not implemented',MB_OK Or
                                         MB_IconInformation);
                  End;
                End;
  End;
  WindowProc := DefWindowProc(Window,AMessage,WParam,LParam);
End;

Function WinRegister: Boolean;
Begin
  With WindowClass Do
    Begin
      Style := cs_hRedraw Or cs_vRedraw;
      lpfnWndProc := WndProc(@WindowProc);
      cbClsExtra := 0;
      cbWndExtra := 0;
      hInstance := system.MainInstance;
      hIcon := LoadIcon (0,idi_Application);
      hCursor := LoadCursor (0,idc_Arrow);
      hbrBackground := GetStockObject(GRAY_BRUSH);
      lpszMenuName := 'Files';
      lpszClassName := AppName;
    End;
  Result := RegisterClass (WindowClass)<>0;
End;

Function EditCreate(ParentWindow,Status:HWnd): HWnd;

Const
  CS_Start = WS_Child or WS_HScroll or WS_VScroll or ES_MultiLine or ES_Left;
  EdiTText: PChar = '';

Var
  HEdit: HWND;
  R: TRect;
  StatH: Word;
{                                             rev 1.5 : comment out
  Procedure AddText (S:String);
  begin
    S:=S+#0;
    SendMessage(HEdit,em_replacesel,0,longint(pchar(@S[1])));
  end;
}
Begin
  GetClientRect(Status,@R);
  StatH := R.Bottom-R.Top;
  GetClientRect(ParentWindow,@R);
  HEdit := CreateWindow ('EDIT',EditText,CS_Start,0,0,
                       R.Right-R.Left,R.Bottom-R.top-StatH,ParentWindow,0,
                       System.MainInstance,Nil);
  If HEdit<>0 Then
    Begin
      ShowWindow(Hedit,cmdShow);
      UpdateWindow(HEdit);
    End;
  Result := HEdit;
End;

Function WinCreate: HWnd;

Var hWindow: HWnd;
    SubMenu: hMenu;
Begin
  hWindow := CreateWindow (AppName,'MenuDemo',ws_OverlappedWindow,
                         cw_UseDefault,cw_UseDefault,cw_UseDefault,
                         cw_UseDefault,0,0,system.MainInstance,Nil);
  If hWindow<>0 Then
    Begin
      Menu := CreateMenu;
      SubMenu := CreateMenu;
      AppendMenu(Submenu,MF_STRING,101,'&New...');
      AppendMenu(Submenu,MF_STRING,102,'&Open...');
      AppendMenu(Submenu,MF_STRING,103,'&Save...');
      AppendMenu(Submenu,MF_SEPARATOR,0,Nil);
      AppendMenu(SubMenu,MF_String,104,'E&xit');
      AppendMenu(Menu,MF_POPUP,SubMenu,'&Files');
      SubMenu := CreateMenu;
      AppendMenu(SubMenu,MF_String,201,'&Cut');
      AppendMenu(SubMenu,MF_String,202,'&Copy');
      AppendMenu(SubMenu,MF_STRING,203,'&Paste');
      AppendMenu(Menu,MF_POPUP,SubMenu,'&Edit');
      SubMenu := CreateMenu;
      AppendMenu(SubMenu,MF_String,301,'&Settings');
      AppendMenu(Menu,MF_POPUP,SubMenu,'&Options');
      AppendMenu(Menu,MF_STRING,0,'&Help');
      SetMenu(hWindow,menu);
      ShowWindow(hWindow,CmdShow);
      UpdateWindow(hWindow);
    End;
  Result := hWindow;
End;

Function StatusCreate (parent:hwnd): HWnd;
Begin
  StatusCreate := CreateStatusWindow (WS_CHILD Or WS_VISIBLE,'Ready...',parent,$7712);
End;

Begin
  If Not WinRegister Then
    Begin
      MessageBox (0,'Register failed',Nil, mb_Ok);
    End
  Else
    Begin
      hWindow := WinCreate;
      If longint(hWindow)=0 Then
        Begin
          MessageBox (0,'WinCreate failed',Nil,MB_OK);
        End
      Else
        Begin
          HStatus := statuscreate(hwindow);
          HEdit := EditCreate(HWindow,HStatus);
          While GetMessage(@AMessage,0,0,0) Do
          Begin
            TranslateMessage(AMessage);
            DispatchMessage(AMessage);
          End;
          Halt(AMessage.wParam);
        End;
    End;
End.

{
  $Log: menu.pp,v $
  Revision 1.7  2000/01/21 00:44:51  peter
    * remove unused vars
    * renamed to .pp

  Revision 1.6  1999/08/10 08:21:45  michael
  + Log keyword placement correction.

  Revision 1.5  1999/08/10 08:20:49  michael
   + Enhancements by G�ran Andersson

  Revision 1.4  1999/07/16 12:20:57  michael
  + Added saving in fixed file

  Revision 1.3  1999/06/28 16:15:11  peter
    * fixed dup id

  Revision 1.2  1999/05/03 18:04:39  peter
    * updates

}
