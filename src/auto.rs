/// The part of the widget theme to retrieve.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Part {
    /// The `BP_CHECKBOX` part.
    Checkbox(Checkbox),
    /// The `BP_COMMANDLINK` part.
    Commandlink(Commandlink),
    /// The `BP_COMMANDLINKGLYPH` part.
    Commandlinkglyph(Commandlinkglyph),
    /// The `BP_GROUPBOX` part.
    Groupbox(Groupbox),
    /// The `BP_PUSHBUTTON` part.
    Pushbutton(Pushbutton),
    /// The `BP_RADIOBUTTON` part.
    Radiobutton(Radiobutton),
    /// The `CLP_TIME` part.
    Time(Time),
    /// The `COMBOBOX` part.
    Combobox(Combobox),
    /// The `CP_BORDER` part.
    Border(Border),
    /// The `CP_CUEBANNER` part.
    Cuebanner(Cuebanner),
    /// The `CP_DROPDOWNBUTTON` part.
    Dropdownbutton(Dropdownbutton),
    /// The `CP_DROPDOWNBUTTONLEFT` part.
    Dropdownbuttonleft(Dropdownbuttonleft),
    /// The `CP_DROPDOWNBUTTONRIGHT` part.
    Dropdownbuttonright(Dropdownbuttonright),
    /// The `CP_TRANSPARENTBACKGROUND` part.
    Transparentbackground(Transparentbackground),
    /// The `CP_READONLY` part.
    Readonly(Readonly),
    /// The `CSST_TAB` part.
    Tab(Tab),
    /// The `CONTROLPANEL` part.
    Controlpanel(Controlpanel),
    /// The `CPANEL_CONTENTLINK` part.
    Contentlink(Contentlink),
    /// The `CPANEL_HELPLINK` part.
    Helplink(Helplink),
    /// The `CPANEL_SECTIONTITLELINK` part.
    Sectiontitlelink(Sectiontitlelink),
    /// The `CPANEL_TASKLINK` part.
    Tasklink(Tasklink),
    /// The `DP_DATEBORDER` part.
    Dateborder(Dateborder),
    /// The `DP_DATETEXT` part.
    Datetext(Datetext),
    /// The `DP_SHOWCALENDARBUTTONRIGHT` part.
    Showcalendarbuttonright(Showcalendarbuttonright),
    /// The `DD_COPY` part.
    Copy(Copy),
    /// The `DD_CREATELINK` part.
    Createlink(Createlink),
    /// The `DD_MOVE` part.
    Move(Move),
    /// The `DD_NONE` part.
    None(None),
    /// The `DD_UPDATEMETADATA` part.
    Updatemetadata(Updatemetadata),
    /// The `DD_WARNING` part.
    Warning(Warning),
    /// The `EP_BACKGROUND` part.
    Background(Background),
    /// The `EP_BACKGROUNDWITHBORDER` part.
    Backgroundwithborder(Backgroundwithborder),
    /// The `EP_EDITBORDER_HSCROLL` part.
    Editborder(Editborder),
    /// The `EP_EDITBORDER_HVSCROLL` part.
    Editborder(Editborder),
    /// The `EP_EDITBORDER_NOSCROLL` part.
    Editborder(Editborder),
    /// The `EP_EDITBORDER_VSCROLL` part.
    Editborder(Editborder),
    /// The `EP_EDITTEXT` part.
    Edittext(Edittext),
    /// The `EXPLORERBAR` part.
    Explorerbar(Explorerbar),
    /// The `EBP_HEADERCLOSE` part.
    Headerclose(Headerclose),
    /// The `EBP_HEADERPIN` part.
    Headerpin(Headerpin),
    /// The `EBP_IEBARMENU` part.
    Iebarmenu(Iebarmenu),
    /// The `EBP_NORMALGROUPCOLLAPSE` part.
    Normalgroupcollapse(Normalgroupcollapse),
    /// The `EBP_NORMALGROUPEXPAND` part.
    Normalgroupexpand(Normalgroupexpand),
    /// The `EBP_SPECIALGROUPCOLLAPSE` part.
    Specialgroupcollapse(Specialgroupcollapse),
    /// The `EBP_SPECIALGROUPEXPAND` part.
    Specialgroupexpand(Specialgroupexpand),
    /// The `FLYOUT_BODY` part.
    Body(Body),
    /// The `FLYOUT_LABEL` part.
    Label(Label),
    /// The `FLYOUT_LINK` part.
    Link(Link),
    /// The `FLYOUT_LINKHEADER` part.
    Linkheader(Linkheader),
    /// The `GP_BORDER` part.
    Border(Border),
    /// The `GP_LINEHORZ` part.
    Linehorz(Linehorz),
    /// The `GP_LINEVERT` part.
    Linevert(Linevert),
    /// The `HP_HEADERDROPDOWN` part.
    Headerdropdown(Headerdropdown),
    /// The `HP_HEADERDROPDOWNFILTER` part.
    Headerdropdownfilter(Headerdropdownfilter),
    /// The `HP_HEADERITEM` part.
    Headeritem(Headeritem),
    /// The `HP_HEADERITEMLEFT` part.
    Headeritemleft(Headeritemleft),
    /// The `HP_HEADERITEMRIGHT` part.
    Headeritemright(Headeritemright),
    /// The `HP_HEADEROVERFLOW` part.
    Headeroverflow(Headeroverflow),
    /// The `HP_HEADERSORTARROW` part.
    Headersortarrow(Headersortarrow),
    /// The `LBCP_BORDER_HSCROLL` part.
    Border(Border),
    /// The `LBCP_BORDER_HVSCROLL` part.
    Border(Border),
    /// The `LBCP_BORDER_NOSCROLL` part.
    Border(Border),
    /// The `LBCP_BORDER_VSCROLL` part.
    Border(Border),
    /// The `LBCP_ITEM` part.
    Item(Item),
    /// The `LVP_COLLAPSEBUTTON` part.
    Collapsebutton(Collapsebutton),
    /// The `LVP_EXPANDBUTTON` part.
    Expandbutton(Expandbutton),
    /// The `LVP_GROUPHEADER` part.
    Groupheader(Groupheader),
    /// The `LVP_GROUPHEADERLINE` part.
    Groupheaderline(Groupheaderline),
    /// The `LVP_LISTITEM` part.
    Listitem(Listitem),
    /// The `MENU_BARBACKGROUND` part.
    Barbackground(Barbackground),
    /// The `MENU_BARITEM` part.
    Baritem(Baritem),
    /// The `MENU_POPUPCHECK` part.
    Popupcheck(Popupcheck),
    /// The `MENU_POPUPCHECKBACKGROUND` part.
    Popupcheckbackground(Popupcheckbackground),
    /// The `MENU_POPUPITEM` part.
    Popupitem(Popupitem),
    /// The `MENU_POPUPITEM_FOCUSABLE` part.
    Popupitem(Popupitem),
    /// The `MENU_POPUPITEMKBFOCUS` part.
    Popupitemkbfocus(Popupitemkbfocus),
    /// The `MENU_POPUPSUBMENU` part.
    Popupsubmenu(Popupsubmenu),
    /// The `MENU_SYSTEMCLOSE` part.
    Systemclose(Systemclose),
    /// The `MENU_SYSTEMMAXIMIZE` part.
    Systemmaximize(Systemmaximize),
    /// The `MENU_SYSTEMMINIMIZE` part.
    Systemminimize(Systemminimize),
    /// The `MENU_SYSTEMRESTORE` part.
    Systemrestore(Systemrestore),
    /// The `MDP_NEWAPPBUTTON` part.
    Newappbutton(Newappbutton),
    /// The `NAV_BACKBUTTON` part.
    Backbutton(Backbutton),
    /// The `NAV_FORWARDBUTTON` part.
    Forwardbutton(Forwardbutton),
    /// The `NAV_MENUBUTTON` part.
    Menubutton(Menubutton),
    /// The `PGRP_DOWN` part.
    Down(Down),
    /// The `PGRP_DOWNHORZ` part.
    Downhorz(Downhorz),
    /// The `PGRP_UP` part.
    Up(Up),
    /// The `PGRP_UPHORZ` part.
    Uphorz(Uphorz),
    /// The `PROGRESS` part.
    Progress(Progress),
    /// The `PP_FILL` part.
    Fill(Fill),
    /// The `PP_FILLVERT` part.
    Fillvert(Fillvert),
    /// The `PP_TRANSPARENTBAR` part.
    Transparentbar(Transparentbar),
    /// The `PP_TRANSPARENTBARVERT` part.
    Transparentbarvert(Transparentbarvert),
    /// The `REBAR` part.
    Rebar(Rebar),
    /// The `RP_CHEVRON` part.
    Chevron(Chevron),
    /// The `RP_CHEVRONVERT` part.
    Chevronvert(Chevronvert),
    /// The `RP_SPLITTER` part.
    Splitter(Splitter),
    /// The `RP_SPLITTERVERT` part.
    Splittervert(Splittervert),
    /// The `SBP_ARROWBTN` part.
    Arrowbtn(Arrowbtn),
    /// The `SBP_GRIPPERHORZ` part.
    Gripperhorz(Gripperhorz),
    /// The `SBP_GRIPPERVERT` part.
    Grippervert(Grippervert),
    /// The `SBP_LOWERTRACKHORZ` part.
    Lowertrackhorz(Lowertrackhorz),
    /// The `SBP_LOWERTRACKVERT` part.
    Lowertrackvert(Lowertrackvert),
    /// The `SBP_SIZEBOX` part.
    Sizebox(Sizebox),
    /// The `SBP_THUMBBTNHORZ` part.
    Thumbbtnhorz(Thumbbtnhorz),
    /// The `SBP_THUMBBTNVERT` part.
    Thumbbtnvert(Thumbbtnvert),
    /// The `SBP_UPPERTRACKHORZ` part.
    Uppertrackhorz(Uppertrackhorz),
    /// The `SBP_UPPERTRACKVERT` part.
    Uppertrackvert(Uppertrackvert),
    /// The `SEBP_SEARCHEDITBOXTEXT` part.
    Searcheditboxtext(Searcheditboxtext),
    /// The `SPNP_DOWN` part.
    Down(Down),
    /// The `SPNP_DOWNHORZ` part.
    Downhorz(Downhorz),
    /// The `SPNP_UP` part.
    Up(Up),
    /// The `SPNP_UPHORZ` part.
    Uphorz(Uphorz),
    /// The `STARTPANEL` part.
    Startpanel(Startpanel),
    /// The `SPP_LOGOFFBUTTONS` part.
    Logoffbuttons(Logoffbuttons),
    /// The `SPP_MOREPROGRAMSARROW` part.
    Moreprogramsarrow(Moreprogramsarrow),
    /// The `STATUS` part.
    Status(Status),
    /// The `TAB` part.
    Tab(Tab),
    /// The `TABP_TABITEM` part.
    Tabitem(Tabitem),
    /// The `TABP_TABITEMBOTHEDGE` part.
    Tabitembothedge(Tabitembothedge),
    /// The `TABP_TABITEMLEFTEDGE` part.
    Tabitemleftedge(Tabitemleftedge),
    /// The `TABP_TABITEMRIGHTEDGE` part.
    Tabitemrightedge(Tabitemrightedge),
    /// The `TABP_TOPTABITEM` part.
    Toptabitem(Toptabitem),
    /// The `TABP_TOPTABITEMBOTHEDGE` part.
    Toptabitembothedge(Toptabitembothedge),
    /// The `TABP_TOPTABITEMLEFTEDGE` part.
    Toptabitemleftedge(Toptabitemleftedge),
    /// The `TABP_TOPTABITEMRIGHTEDGE` part.
    Toptabitemrightedge(Toptabitemrightedge),
    /// The `TASKBAND` part.
    Taskband(Taskband),
    /// The `TASKBAR` part.
    Taskbar(Taskbar),
    /// The `TASKDIALOG` part.
    Taskdialog(Taskdialog),
    /// The `TDLG_CONTENTPANE` part.
    Contentpane(Contentpane),
    /// The `TDLG_EXPANDOBUTTON` part.
    Expandobutton(Expandobutton),
    /// The `TEXTSTYLE` part.
    Textstyle(Textstyle),
    /// The `TEXT_CONTROLLABEL` part.
    Controllabel(Controllabel),
    /// The `TEXT_HYPERLINKTEXT` part.
    Hyperlinktext(Hyperlinktext),
    /// The `TP_BUTTON` part.
    Button(Button),
    /// The `TP_DROPDOWNBUTTON` part.
    Dropdownbutton(Dropdownbutton),
    /// The `TP_DROPDOWNBUTTONGLYPH` part.
    Dropdownbuttonglyph(Dropdownbuttonglyph),
    /// The `TP_SEPARATOR` part.
    Separator(Separator),
    /// The `TP_SEPARATORVERT` part.
    Separatorvert(Separatorvert),
    /// The `TP_SPLITBUTTON` part.
    Splitbutton(Splitbutton),
    /// The `TP_SPLITBUTTONDROPDOWN` part.
    Splitbuttondropdown(Splitbuttondropdown),
    /// The `TTP_BALLOON` part.
    Balloon(Balloon),
    /// The `TTP_BALLOONSTEM` part.
    Balloonstem(Balloonstem),
    /// The `TTP_CLOSE` part.
    Close(Close),
    /// The `TTP_STANDARD` part.
    Standard(Standard),
    /// The `TTP_STANDARDTITLE` part.
    Standardtitle(Standardtitle),
    /// The `TTP_WRENCH` part.
    Wrench(Wrench),
    /// The `TKP_THUMB` part.
    Thumb(Thumb),
    /// The `TKP_THUMBBOTTOM` part.
    Thumbbottom(Thumbbottom),
    /// The `TKP_THUMBLEFT` part.
    Thumbleft(Thumbleft),
    /// The `TKP_THUMBRIGHT` part.
    Thumbright(Thumbright),
    /// The `TKP_THUMBTOP` part.
    Thumbtop(Thumbtop),
    /// The `TKP_THUMBVERT` part.
    Thumbvert(Thumbvert),
    /// The `TKP_TICS` part.
    Tics(Tics),
    /// The `TKP_TICSVERT` part.
    Ticsvert(Ticsvert),
    /// The `TKP_TRACK` part.
    Track(Track),
    /// The `TKP_TRACKVERT` part.
    Trackvert(Trackvert),
    /// The `TRAYNOTIFY` part.
    Traynotify(Traynotify),
    /// The `TREEVIEW` part.
    Treeview(Treeview),
    /// The `TVP_GLYPH` part.
    Glyph(Glyph),
    /// The `TVP_HOTGLYPH` part.
    Hotglyph(Hotglyph),
    /// The `TVP_TREEITEM` part.
    Treeitem(Treeitem),
    /// The `WP_CAPTION` part.
    Caption(Caption),
    /// The `WP_CLOSEBUTTON` part.
    Closebutton(Closebutton),
    /// The `WP_FRAME` part.
    Frame(Frame),
    /// The `WP_HELPBUTTON` part.
    Helpbutton(Helpbutton),
    /// The `WP_HORZSCROLL` part.
    Horzscroll(Horzscroll),
    /// The `WP_HORZTHUMB` part.
    Horzthumb(Horzthumb),
    /// The `WP_MAXBUTTON` part.
    Maxbutton(Maxbutton),
    /// The `WP_MAXCAPTION` part.
    Maxcaption(Maxcaption),
    /// The `WP_MINBUTTON` part.
    Minbutton(Minbutton),
    /// The `WP_MINCAPTION` part.
    Mincaption(Mincaption),
    /// The `WP_RESTOREBUTTON` part.
    Restorebutton(Restorebutton),
    /// The `WP_SYSBUTTON` part.
    Sysbutton(Sysbutton),
    /// The `WP_VERTSCROLL` part.
    Vertscroll(Vertscroll),
    /// The `WP_VERTTHUMB` part.
    Vertthumb(Vertthumb),
}
/// The state of the `BP_CHECKBOX` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Checkbox {
    /// The default state.
    None = 0,
    /// The `CBS_CHECKEDDISABLED` state.
    Checkeddisabled = windows_sys::Win32::UI::Controls::CBS_CHECKEDDISABLED,
}
/// The state of the `BP_COMMANDLINK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Commandlink {
    /// The default state.
    None = 0,
    /// The `CMDLS_DEFAULTED` state.
    Defaulted = windows_sys::Win32::UI::Controls::CMDLS_DEFAULTED,
}
/// The state of the `BP_COMMANDLINKGLYPH` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Commandlinkglyph {
    /// The default state.
    None = 0,
    /// The `CMDLGS_DEFAULTED` state.
    Defaulted = windows_sys::Win32::UI::Controls::CMDLGS_DEFAULTED,
}
/// The state of the `BP_GROUPBOX` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Groupbox {
    /// The default state.
    None = 0,
    /// The `GBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::GBS_DISABLED,
}
/// The state of the `BP_PUSHBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Pushbutton {
    /// The default state.
    None = 0,
    /// The `PBS_DEFAULTED` state.
    Defaulted = windows_sys::Win32::UI::Controls::PBS_DEFAULTED,
}
/// The state of the `BP_RADIOBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Radiobutton {
    /// The default state.
    None = 0,
    /// The `RBS_CHECKEDDISABLED` state.
    Checkeddisabled = windows_sys::Win32::UI::Controls::RBS_CHECKEDDISABLED,
    /// The `BP_USERBUTTON` state.
    Userbutton = windows_sys::Win32::UI::Controls::BP_USERBUTTON,
}
/// The state of the `CLP_TIME` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Time {
    /// The default state.
    None = 0,
    /// The `CLS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::CLS_NORMAL,
}
/// The state of the `COMBOBOX` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Combobox {
    /// The default state.
    None = 0,
    /// The `CP_BACKGROUND` state.
    Background = windows_sys::Win32::UI::Controls::CP_BACKGROUND,
}
/// The state of the `CP_BORDER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Border {
    /// The default state.
    None = 0,
    /// The `CBB_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CBB_DISABLED,
}
/// The state of the `CP_CUEBANNER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Cuebanner {
    /// The default state.
    None = 0,
    /// The `CBCB_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CBCB_DISABLED,
}
/// The state of the `CP_DROPDOWNBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Dropdownbutton {
    /// The default state.
    None = 0,
    /// The `CBXS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CBXS_DISABLED,
}
/// The state of the `CP_DROPDOWNBUTTONLEFT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Dropdownbuttonleft {
    /// The default state.
    None = 0,
    /// The `CBXSL_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CBXSL_DISABLED,
}
/// The state of the `CP_DROPDOWNBUTTONRIGHT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Dropdownbuttonright {
    /// The default state.
    None = 0,
    /// The `CBXSR_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CBXSR_DISABLED,
}
/// The state of the `CP_TRANSPARENTBACKGROUND` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Transparentbackground {
    /// The default state.
    None = 0,
    /// The `CBTBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CBTBS_DISABLED,
}
/// The state of the `CP_READONLY` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Readonly {
    /// The default state.
    None = 0,
    /// The `CBRO_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CBRO_DISABLED,
}
/// The state of the `CSST_TAB` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Tab {
    /// The default state.
    None = 0,
    /// The `CSTB_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::CSTB_HOT,
}
/// The state of the `CONTROLPANEL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Controlpanel {
    /// The default state.
    None = 0,
    /// The `CPANEL_BANNERAREA` state.
    Bannerarea = windows_sys::Win32::UI::Controls::CPANEL_BANNERAREA,
    /// The `CPANEL_BODYTEXT` state.
    Bodytext = windows_sys::Win32::UI::Controls::CPANEL_BODYTEXT,
    /// The `CPANEL_BODYTITLE` state.
    Bodytitle = windows_sys::Win32::UI::Controls::CPANEL_BODYTITLE,
    /// The `CPANEL_BUTTON` state.
    Button = windows_sys::Win32::UI::Controls::CPANEL_BUTTON,
}
/// The state of the `CPANEL_CONTENTLINK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Contentlink {
    /// The default state.
    None = 0,
    /// The `CPCL_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CPCL_DISABLED,
    /// The `CPANEL_CONTENTPANE` state.
    Contentpane = windows_sys::Win32::UI::Controls::CPANEL_CONTENTPANE,
    /// The `CPANEL_CONTENTPANELABEL` state.
    Contentpanelabel = windows_sys::Win32::UI::Controls::CPANEL_CONTENTPANELABEL,
    /// The `CPANEL_CONTENTPANELINE` state.
    Contentpaneline = windows_sys::Win32::UI::Controls::CPANEL_CONTENTPANELINE,
    /// The `CPANEL_GROUPTEXT` state.
    Grouptext = windows_sys::Win32::UI::Controls::CPANEL_GROUPTEXT,
}
/// The state of the `CPANEL_HELPLINK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Helplink {
    /// The default state.
    None = 0,
    /// The `CPHL_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CPHL_DISABLED,
    /// The `CPANEL_LARGECOMMANDAREA` state.
    Largecommandarea = windows_sys::Win32::UI::Controls::CPANEL_LARGECOMMANDAREA,
    /// The `CPANEL_MESSAGETEXT` state.
    Messagetext = windows_sys::Win32::UI::Controls::CPANEL_MESSAGETEXT,
    /// The `CPANEL_NAVIGATIONPANE` state.
    Navigationpane = windows_sys::Win32::UI::Controls::CPANEL_NAVIGATIONPANE,
    /// The `CPANEL_NAVIGATIONPANELABEL` state.
    Navigationpanelabel = windows_sys::Win32::UI::Controls::CPANEL_NAVIGATIONPANELABEL,
    /// The `CPANEL_NAVIGATIONPANELINE` state.
    Navigationpaneline = windows_sys::Win32::UI::Controls::CPANEL_NAVIGATIONPANELINE,
}
/// The state of the `CPANEL_SECTIONTITLELINK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Sectiontitlelink {
    /// The default state.
    None = 0,
    /// The `CPSTL_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::CPSTL_HOT,
    /// The `CPANEL_SMALLCOMMANDAREA` state.
    Smallcommandarea = windows_sys::Win32::UI::Controls::CPANEL_SMALLCOMMANDAREA,
}
/// The state of the `CPANEL_TASKLINK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Tasklink {
    /// The default state.
    None = 0,
    /// The `CPTL_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CPTL_DISABLED,
    /// The `CPANEL_TITLE` state.
    Title = windows_sys::Win32::UI::Controls::CPANEL_TITLE,
}
/// The state of the `DP_DATEBORDER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Dateborder {
    /// The default state.
    None = 0,
    /// The `DPDB_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::DPDB_DISABLED,
}
/// The state of the `DP_DATETEXT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Datetext {
    /// The default state.
    None = 0,
    /// The `DPDT_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::DPDT_DISABLED,
}
/// The state of the `DP_SHOWCALENDARBUTTONRIGHT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Showcalendarbuttonright {
    /// The default state.
    None = 0,
    /// The `DPSCBR_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::DPSCBR_DISABLED,
}
/// The state of the `DD_COPY` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Copy {
    /// The default state.
    None = 0,
    /// The `DDCOPY_HIGHLIGHT` state.
    Highlight = windows_sys::Win32::UI::Controls::DDCOPY_HIGHLIGHT,
}
/// The state of the `DD_CREATELINK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Createlink {
    /// The default state.
    None = 0,
    /// The `DDCREATELINK_HIGHLIGHT` state.
    Highlight = windows_sys::Win32::UI::Controls::DDCREATELINK_HIGHLIGHT,
    /// The `DD_IMAGEBG` state.
    Imagebg = windows_sys::Win32::UI::Controls::DD_IMAGEBG,
}
/// The state of the `DD_MOVE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Move {
    /// The default state.
    None = 0,
    /// The `DDMOVE_HIGHLIGHT` state.
    Highlight = windows_sys::Win32::UI::Controls::DDMOVE_HIGHLIGHT,
}
/// The state of the `DD_NONE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum None {
    /// The default state.
    None = 0,
    /// The `DDNONE_HIGHLIGHT` state.
    Highlight = windows_sys::Win32::UI::Controls::DDNONE_HIGHLIGHT,
    /// The `DD_TEXTBG` state.
    Textbg = windows_sys::Win32::UI::Controls::DD_TEXTBG,
}
/// The state of the `DD_UPDATEMETADATA` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Updatemetadata {
    /// The default state.
    None = 0,
    /// The `DDUPDATEMETADATA_HIGHLIGHT` state.
    Highlight = windows_sys::Win32::UI::Controls::DDUPDATEMETADATA_HIGHLIGHT,
}
/// The state of the `DD_WARNING` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Warning {
    /// The default state.
    None = 0,
    /// The `DDWARNING_HIGHLIGHT` state.
    Highlight = windows_sys::Win32::UI::Controls::DDWARNING_HIGHLIGHT,
}
/// The state of the `EP_BACKGROUND` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Background {
    /// The default state.
    None = 0,
    /// The `EBS_ASSIST` state.
    Assist = windows_sys::Win32::UI::Controls::EBS_ASSIST,
}
/// The state of the `EP_BACKGROUNDWITHBORDER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Backgroundwithborder {
    /// The default state.
    None = 0,
    /// The `EBWBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::EBWBS_DISABLED,
    /// The `EP_CARET` state.
    Caret = windows_sys::Win32::UI::Controls::EP_CARET,
}
/// The state of the `EP_EDITBORDER_HSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Editborder {
    /// The default state.
    None = 0,
    /// The `EPSH_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::EPSH_DISABLED,
}
/// The state of the `EP_EDITBORDER_HVSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Editborder {
    /// The default state.
    None = 0,
    /// The `EPSHV_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::EPSHV_DISABLED,
}
/// The state of the `EP_EDITBORDER_NOSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Editborder {
    /// The default state.
    None = 0,
    /// The `EPSN_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::EPSN_DISABLED,
}
/// The state of the `EP_EDITBORDER_VSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Editborder {
    /// The default state.
    None = 0,
    /// The `EPSV_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::EPSV_DISABLED,
}
/// The state of the `EP_EDITTEXT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Edittext {
    /// The default state.
    None = 0,
    /// The `ETS_ASSIST` state.
    Assist = windows_sys::Win32::UI::Controls::ETS_ASSIST,
    /// The `EP_PASSWORD` state.
    Password = windows_sys::Win32::UI::Controls::EP_PASSWORD,
}
/// The state of the `EXPLORERBAR` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Explorerbar {
    /// The default state.
    None = 0,
    /// The `EBP_HEADERBACKGROUND` state.
    Headerbackground = windows_sys::Win32::UI::Controls::EBP_HEADERBACKGROUND,
}
/// The state of the `EBP_HEADERCLOSE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headerclose {
    /// The default state.
    None = 0,
    /// The `EBHC_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::EBHC_HOT,
}
/// The state of the `EBP_HEADERPIN` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headerpin {
    /// The default state.
    None = 0,
    /// The `EBHP_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::EBHP_HOT,
}
/// The state of the `EBP_IEBARMENU` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Iebarmenu {
    /// The default state.
    None = 0,
    /// The `EBM_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::EBM_HOT,
    /// The `EBP_NORMALGROUPBACKGROUND` state.
    Normalgroupbackground = windows_sys::Win32::UI::Controls::EBP_NORMALGROUPBACKGROUND,
}
/// The state of the `EBP_NORMALGROUPCOLLAPSE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Normalgroupcollapse {
    /// The default state.
    None = 0,
    /// The `EBNGC_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::EBNGC_HOT,
}
/// The state of the `EBP_NORMALGROUPEXPAND` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Normalgroupexpand {
    /// The default state.
    None = 0,
    /// The `EBNGE_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::EBNGE_HOT,
    /// The `EBP_NORMALGROUPHEAD` state.
    Normalgrouphead = windows_sys::Win32::UI::Controls::EBP_NORMALGROUPHEAD,
    /// The `EBP_SPECIALGROUPBACKGROUND` state.
    Specialgroupbackground = windows_sys::Win32::UI::Controls::EBP_SPECIALGROUPBACKGROUND,
}
/// The state of the `EBP_SPECIALGROUPCOLLAPSE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Specialgroupcollapse {
    /// The default state.
    None = 0,
    /// The `EBSGC_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::EBSGC_HOT,
}
/// The state of the `EBP_SPECIALGROUPEXPAND` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Specialgroupexpand {
    /// The default state.
    None = 0,
    /// The `EBSGE_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::EBSGE_HOT,
    /// The `EBP_SPECIALGROUPHEAD` state.
    Specialgrouphead = windows_sys::Win32::UI::Controls::EBP_SPECIALGROUPHEAD,
}
/// The state of the `FLYOUT_BODY` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Body {
    /// The default state.
    None = 0,
    /// The `FBS_EMPHASIZED` state.
    Emphasized = windows_sys::Win32::UI::Controls::FBS_EMPHASIZED,
    /// The `FLYOUT_DIVIDER` state.
    Divider = windows_sys::Win32::UI::Controls::FLYOUT_DIVIDER,
    /// The `FLYOUT_HEADER` state.
    Header = windows_sys::Win32::UI::Controls::FLYOUT_HEADER,
}
/// The state of the `FLYOUT_LABEL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Label {
    /// The default state.
    None = 0,
    /// The `FLS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::FLS_DISABLED,
}
/// The state of the `FLYOUT_LINK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Link {
    /// The default state.
    None = 0,
    /// The `FLYOUTLINK_HOVER` state.
    Hover = windows_sys::Win32::UI::Controls::FLYOUTLINK_HOVER,
    /// The `FLYOUT_LINKAREA` state.
    Linkarea = windows_sys::Win32::UI::Controls::FLYOUT_LINKAREA,
}
/// The state of the `FLYOUT_LINKHEADER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Linkheader {
    /// The default state.
    None = 0,
    /// The `FLH_HOVER` state.
    Hover = windows_sys::Win32::UI::Controls::FLH_HOVER,
    /// The `FLYOUT_WINDOW` state.
    Window = windows_sys::Win32::UI::Controls::FLYOUT_WINDOW,
}
/// The state of the `GP_BORDER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Border {
    /// The default state.
    None = 0,
    /// The `BSS_FLAT` state.
    Flat = windows_sys::Win32::UI::Controls::BSS_FLAT,
}
/// The state of the `GP_LINEHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Linehorz {
    /// The default state.
    None = 0,
    /// The `LHS_FLAT` state.
    Flat = windows_sys::Win32::UI::Controls::LHS_FLAT,
}
/// The state of the `GP_LINEVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Linevert {
    /// The default state.
    None = 0,
    /// The `LVS_FLAT` state.
    Flat = windows_sys::Win32::UI::Controls::LVS_FLAT,
}
/// The state of the `HP_HEADERDROPDOWN` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headerdropdown {
    /// The default state.
    None = 0,
    /// The `HDDS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::HDDS_HOT,
}
/// The state of the `HP_HEADERDROPDOWNFILTER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headerdropdownfilter {
    /// The default state.
    None = 0,
    /// The `HDDFS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::HDDFS_HOT,
}
/// The state of the `HP_HEADERITEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headeritem {
    /// The default state.
    None = 0,
    /// The `HIS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::HIS_HOT,
}
/// The state of the `HP_HEADERITEMLEFT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headeritemleft {
    /// The default state.
    None = 0,
    /// The `HILS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::HILS_HOT,
}
/// The state of the `HP_HEADERITEMRIGHT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headeritemright {
    /// The default state.
    None = 0,
    /// The `HIRS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::HIRS_HOT,
}
/// The state of the `HP_HEADEROVERFLOW` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headeroverflow {
    /// The default state.
    None = 0,
    /// The `HOFS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::HOFS_HOT,
}
/// The state of the `HP_HEADERSORTARROW` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Headersortarrow {
    /// The default state.
    None = 0,
    /// The `HSAS_SORTEDDOWN` state.
    Sorteddown = windows_sys::Win32::UI::Controls::HSAS_SORTEDDOWN,
}
/// The state of the `LBCP_BORDER_HSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Border {
    /// The default state.
    None = 0,
    /// The `LBPSH_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::LBPSH_DISABLED,
}
/// The state of the `LBCP_BORDER_HVSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Border {
    /// The default state.
    None = 0,
    /// The `LBPSHV_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::LBPSHV_DISABLED,
}
/// The state of the `LBCP_BORDER_NOSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Border {
    /// The default state.
    None = 0,
    /// The `LBPSN_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::LBPSN_DISABLED,
}
/// The state of the `LBCP_BORDER_VSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Border {
    /// The default state.
    None = 0,
    /// The `LBPSV_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::LBPSV_DISABLED,
}
/// The state of the `LBCP_ITEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Item {
    /// The default state.
    None = 0,
    /// The `LBPSI_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::LBPSI_HOT,
}
/// The state of the `LVP_COLLAPSEBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Collapsebutton {
    /// The default state.
    None = 0,
    /// The `LVCB_HOVER` state.
    Hover = windows_sys::Win32::UI::Controls::LVCB_HOVER,
    /// The `LVP_COLUMNDETAIL` state.
    Columndetail = windows_sys::Win32::UI::Controls::LVP_COLUMNDETAIL,
    /// The `LVP_EMPTYTEXT` state.
    Emptytext = windows_sys::Win32::UI::Controls::LVP_EMPTYTEXT,
}
/// The state of the `LVP_EXPANDBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Expandbutton {
    /// The default state.
    None = 0,
    /// The `LVEB_HOVER` state.
    Hover = windows_sys::Win32::UI::Controls::LVEB_HOVER,
}
/// The state of the `LVP_GROUPHEADER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Groupheader {
    /// The default state.
    None = 0,
    /// The `LVGH_CLOSE` state.
    Close = windows_sys::Win32::UI::Controls::LVGH_CLOSE,
}
/// The state of the `LVP_GROUPHEADERLINE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Groupheaderline {
    /// The default state.
    None = 0,
    /// The `LVGHL_CLOSE` state.
    Close = windows_sys::Win32::UI::Controls::LVGHL_CLOSE,
    /// The `LVP_LISTGROUP` state.
    Listgroup = windows_sys::Win32::UI::Controls::LVP_LISTGROUP,
    /// The `LVP_LISTDETAIL` state.
    Listdetail = windows_sys::Win32::UI::Controls::LVP_LISTDETAIL,
}
/// The state of the `LVP_LISTITEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Listitem {
    /// The default state.
    None = 0,
    /// The `LISS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::LISS_DISABLED,
    /// The `LVP_LISTSORTEDDETAIL` state.
    Listsorteddetail = windows_sys::Win32::UI::Controls::LVP_LISTSORTEDDETAIL,
}
/// The state of the `MENU_BARBACKGROUND` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Barbackground {
    /// The default state.
    None = 0,
    /// The `MB_ACTIVE` state.
    Active = windows_sys::Win32::UI::Controls::MB_ACTIVE,
}
/// The state of the `MENU_BARITEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Baritem {
    /// The default state.
    None = 0,
    /// The `MBI_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MBI_DISABLED,
    /// The `MENU_CHEVRON_TMSCHEMA` state.
    Chevron = windows_sys::Win32::UI::Controls::MENU_CHEVRON_TMSCHEMA,
    /// The `MENU_MENUBARDROPDOWN_TMSCHEMA` state.
    Menubardropdown = windows_sys::Win32::UI::Controls::MENU_MENUBARDROPDOWN_TMSCHEMA,
    /// The `MENU_MENUBARITEM_TMSCHEMA` state.
    Menubaritem = windows_sys::Win32::UI::Controls::MENU_MENUBARITEM_TMSCHEMA,
    /// The `MENU_MENUDROPDOWN_TMSCHEMA` state.
    Menudropdown = windows_sys::Win32::UI::Controls::MENU_MENUDROPDOWN_TMSCHEMA,
    /// The `MENU_MENUITEM_TMSCHEMA` state.
    Menuitem = windows_sys::Win32::UI::Controls::MENU_MENUITEM_TMSCHEMA,
    /// The `MENU_POPUPBACKGROUND` state.
    Popupbackground = windows_sys::Win32::UI::Controls::MENU_POPUPBACKGROUND,
    /// The `MENU_POPUPBORDERS` state.
    Popupborders = windows_sys::Win32::UI::Controls::MENU_POPUPBORDERS,
}
/// The state of the `MENU_POPUPCHECK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Popupcheck {
    /// The default state.
    None = 0,
    /// The `MC_BULLETDISABLED` state.
    Bulletdisabled = windows_sys::Win32::UI::Controls::MC_BULLETDISABLED,
}
/// The state of the `MENU_POPUPCHECKBACKGROUND` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Popupcheckbackground {
    /// The default state.
    None = 0,
    /// The `MCB_BITMAP` state.
    Bitmap = windows_sys::Win32::UI::Controls::MCB_BITMAP,
    /// The `MENU_POPUPGUTTER` state.
    Popupgutter = windows_sys::Win32::UI::Controls::MENU_POPUPGUTTER,
}
/// The state of the `MENU_POPUPITEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Popupitem {
    /// The default state.
    None = 0,
    /// The `MPI_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MPI_DISABLED,
}
/// The state of the `MENU_POPUPITEM_FOCUSABLE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Popupitem {
    /// The default state.
    None = 0,
    /// The `MPIF_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::MPIF_NORMAL,
}
/// The state of the `MENU_POPUPITEMKBFOCUS` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Popupitemkbfocus {
    /// The default state.
    None = 0,
    /// The `MPIKBFOCUS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::MPIKBFOCUS_NORMAL,
    /// The `MENU_POPUPSEPARATOR` state.
    Popupseparator = windows_sys::Win32::UI::Controls::MENU_POPUPSEPARATOR,
}
/// The state of the `MENU_POPUPSUBMENU` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Popupsubmenu {
    /// The default state.
    None = 0,
    /// The `MSM_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MSM_DISABLED,
    /// The `MENU_SEPARATOR_TMSCHEMA` state.
    Separator = windows_sys::Win32::UI::Controls::MENU_SEPARATOR_TMSCHEMA,
}
/// The state of the `MENU_SYSTEMCLOSE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Systemclose {
    /// The default state.
    None = 0,
    /// The `MSYSC_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MSYSC_DISABLED,
}
/// The state of the `MENU_SYSTEMMAXIMIZE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Systemmaximize {
    /// The default state.
    None = 0,
    /// The `MSYSMX_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MSYSMX_DISABLED,
}
/// The state of the `MENU_SYSTEMMINIMIZE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Systemminimize {
    /// The default state.
    None = 0,
    /// The `MSYSMN_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MSYSMN_DISABLED,
}
/// The state of the `MENU_SYSTEMRESTORE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Systemrestore {
    /// The default state.
    None = 0,
    /// The `MSYSR_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MSYSR_DISABLED,
}
/// The state of the `MDP_NEWAPPBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Newappbutton {
    /// The default state.
    None = 0,
    /// The `MDS_CHECKED` state.
    Checked = windows_sys::Win32::UI::Controls::MDS_CHECKED,
    /// The `MDP_SEPERATOR` state.
    Seperator = windows_sys::Win32::UI::Controls::MDP_SEPERATOR,
}
/// The state of the `NAV_BACKBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Backbutton {
    /// The default state.
    None = 0,
    /// The `NAV_BB_DISABLED` state.
    Bb = windows_sys::Win32::UI::Controls::NAV_BB_DISABLED,
}
/// The state of the `NAV_FORWARDBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Forwardbutton {
    /// The default state.
    None = 0,
    /// The `NAV_FB_DISABLED` state.
    Fb = windows_sys::Win32::UI::Controls::NAV_FB_DISABLED,
}
/// The state of the `NAV_MENUBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Menubutton {
    /// The default state.
    None = 0,
    /// The `NAV_MB_DISABLED` state.
    Mb = windows_sys::Win32::UI::Controls::NAV_MB_DISABLED,
}
/// The state of the `PGRP_DOWN` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Down {
    /// The default state.
    None = 0,
    /// The `DNS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::DNS_DISABLED,
}
/// The state of the `PGRP_DOWNHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Downhorz {
    /// The default state.
    None = 0,
    /// The `DNHZS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::DNHZS_DISABLED,
}
/// The state of the `PGRP_UP` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Up {
    /// The default state.
    None = 0,
    /// The `UPS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::UPS_DISABLED,
}
/// The state of the `PGRP_UPHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Uphorz {
    /// The default state.
    None = 0,
    /// The `UPHZS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::UPHZS_DISABLED,
}
/// The state of the `PROGRESS` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Progress {
    /// The default state.
    None = 0,
    /// The `PP_BAR` state.
    Bar = windows_sys::Win32::UI::Controls::PP_BAR,
    /// The `PP_BARVERT` state.
    Barvert = windows_sys::Win32::UI::Controls::PP_BARVERT,
    /// The `PP_CHUNK` state.
    Chunk = windows_sys::Win32::UI::Controls::PP_CHUNK,
    /// The `PP_CHUNKVERT` state.
    Chunkvert = windows_sys::Win32::UI::Controls::PP_CHUNKVERT,
}
/// The state of the `PP_FILL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Fill {
    /// The default state.
    None = 0,
    /// The `PBFS_ERROR` state.
    Error = windows_sys::Win32::UI::Controls::PBFS_ERROR,
}
/// The state of the `PP_FILLVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Fillvert {
    /// The default state.
    None = 0,
    /// The `PBFVS_ERROR` state.
    Error = windows_sys::Win32::UI::Controls::PBFVS_ERROR,
    /// The `PP_MOVEOVERLAY` state.
    Moveoverlay = windows_sys::Win32::UI::Controls::PP_MOVEOVERLAY,
    /// The `PP_MOVEOVERLAYVERT` state.
    Moveoverlayvert = windows_sys::Win32::UI::Controls::PP_MOVEOVERLAYVERT,
    /// The `PP_PULSEOVERLAY` state.
    Pulseoverlay = windows_sys::Win32::UI::Controls::PP_PULSEOVERLAY,
    /// The `PP_PULSEOVERLAYVERT` state.
    Pulseoverlayvert = windows_sys::Win32::UI::Controls::PP_PULSEOVERLAYVERT,
}
/// The state of the `PP_TRANSPARENTBAR` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Transparentbar {
    /// The default state.
    None = 0,
    /// The `PBBS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::PBBS_NORMAL,
}
/// The state of the `PP_TRANSPARENTBARVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Transparentbarvert {
    /// The default state.
    None = 0,
    /// The `PBBVS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::PBBVS_NORMAL,
}
/// The state of the `REBAR` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Rebar {
    /// The default state.
    None = 0,
    /// The `RP_BACKGROUND` state.
    Background = windows_sys::Win32::UI::Controls::RP_BACKGROUND,
    /// The `RP_BAND` state.
    Band = windows_sys::Win32::UI::Controls::RP_BAND,
}
/// The state of the `RP_CHEVRON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Chevron {
    /// The default state.
    None = 0,
    /// The `CHEVS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::CHEVS_HOT,
}
/// The state of the `RP_CHEVRONVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Chevronvert {
    /// The default state.
    None = 0,
    /// The `CHEVSV_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::CHEVSV_HOT,
    /// The `RP_GRIPPER` state.
    Gripper = windows_sys::Win32::UI::Controls::RP_GRIPPER,
    /// The `RP_GRIPPERVERT` state.
    Grippervert = windows_sys::Win32::UI::Controls::RP_GRIPPERVERT,
}
/// The state of the `RP_SPLITTER` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Splitter {
    /// The default state.
    None = 0,
    /// The `SPLITS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::SPLITS_HOT,
}
/// The state of the `RP_SPLITTERVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Splittervert {
    /// The default state.
    None = 0,
    /// The `SPLITSV_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::SPLITSV_HOT,
}
/// The state of the `SBP_ARROWBTN` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Arrowbtn {
    /// The default state.
    None = 0,
    /// The `ABS_DOWNDISABLED` state.
    Downdisabled = windows_sys::Win32::UI::Controls::ABS_DOWNDISABLED,
}
/// The state of the `SBP_GRIPPERHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Gripperhorz {
    /// The default state.
    None = 0,
    /// The `SCRBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
}
/// The state of the `SBP_GRIPPERVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Grippervert {
    /// The default state.
    None = 0,
    /// The `SCRBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
}
/// The state of the `SBP_LOWERTRACKHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Lowertrackhorz {
    /// The default state.
    None = 0,
    /// The `SCRBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
}
/// The state of the `SBP_LOWERTRACKVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Lowertrackvert {
    /// The default state.
    None = 0,
    /// The `SCRBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
}
/// The state of the `SBP_SIZEBOX` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Sizebox {
    /// The default state.
    None = 0,
    /// The `SZB_HALFBOTTOMRIGHTALIGN` state.
    Halfbottomrightalign = windows_sys::Win32::UI::Controls::SZB_HALFBOTTOMRIGHTALIGN,
}
/// The state of the `SBP_THUMBBTNHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Thumbbtnhorz {
    /// The default state.
    None = 0,
    /// The `SCRBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
}
/// The state of the `SBP_THUMBBTNVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Thumbbtnvert {
    /// The default state.
    None = 0,
    /// The `SCRBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
}
/// The state of the `SBP_UPPERTRACKHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Uppertrackhorz {
    /// The default state.
    None = 0,
    /// The `SCRBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
}
/// The state of the `SBP_UPPERTRACKVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Uppertrackvert {
    /// The default state.
    None = 0,
    /// The `SCRBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
}
/// The state of the `SEBP_SEARCHEDITBOXTEXT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Searcheditboxtext {
    /// The default state.
    None = 0,
    /// The `SEBTS_FORMATTED` state.
    Formatted = windows_sys::Win32::UI::Controls::SEBTS_FORMATTED,
}
/// The state of the `SPNP_DOWN` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Down {
    /// The default state.
    None = 0,
    /// The `DNS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::DNS_DISABLED,
}
/// The state of the `SPNP_DOWNHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Downhorz {
    /// The default state.
    None = 0,
    /// The `DNHZS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::DNHZS_DISABLED,
}
/// The state of the `SPNP_UP` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Up {
    /// The default state.
    None = 0,
    /// The `UPS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::UPS_DISABLED,
}
/// The state of the `SPNP_UPHORZ` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Uphorz {
    /// The default state.
    None = 0,
    /// The `UPHZS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::UPHZS_DISABLED,
}
/// The state of the `STARTPANEL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Startpanel {
    /// The default state.
    None = 0,
    /// The `SPP_LOGOFF` state.
    Logoff = windows_sys::Win32::UI::Controls::SPP_LOGOFF,
}
/// The state of the `SPP_LOGOFFBUTTONS` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Logoffbuttons {
    /// The default state.
    None = 0,
    /// The `SPLS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::SPLS_HOT,
    /// The `SPP_MOREPROGRAMS` state.
    Moreprograms = windows_sys::Win32::UI::Controls::SPP_MOREPROGRAMS,
}
/// The state of the `SPP_MOREPROGRAMSARROW` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Moreprogramsarrow {
    /// The default state.
    None = 0,
    /// The `SPS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::SPS_HOT,
    /// The `SPP_PLACESLIST` state.
    Placeslist = windows_sys::Win32::UI::Controls::SPP_PLACESLIST,
    /// The `SPP_PLACESLISTSEPARATOR` state.
    Placeslistseparator = windows_sys::Win32::UI::Controls::SPP_PLACESLISTSEPARATOR,
    /// The `SPP_PREVIEW` state.
    Preview = windows_sys::Win32::UI::Controls::SPP_PREVIEW,
    /// The `SPP_PROGLIST` state.
    Proglist = windows_sys::Win32::UI::Controls::SPP_PROGLIST,
    /// The `SPP_PROGLISTSEPARATOR` state.
    Proglistseparator = windows_sys::Win32::UI::Controls::SPP_PROGLISTSEPARATOR,
    /// The `SPP_USERPANE` state.
    Userpane = windows_sys::Win32::UI::Controls::SPP_USERPANE,
    /// The `SPP_USERPICTURE` state.
    Userpicture = windows_sys::Win32::UI::Controls::SPP_USERPICTURE,
}
/// The state of the `STATUS` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Status {
    /// The default state.
    None = 0,
    /// The `SP_GRIPPER` state.
    Gripper = windows_sys::Win32::UI::Controls::SP_GRIPPER,
    /// The `SP_GRIPPERPANE` state.
    Gripperpane = windows_sys::Win32::UI::Controls::SP_GRIPPERPANE,
    /// The `SP_PANE` state.
    Pane = windows_sys::Win32::UI::Controls::SP_PANE,
}
/// The state of the `TAB` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Tab {
    /// The default state.
    None = 0,
    /// The `TABP_AEROWIZARDBODY` state.
    Aerowizardbody = windows_sys::Win32::UI::Controls::TABP_AEROWIZARDBODY,
    /// The `TABP_BODY` state.
    Body = windows_sys::Win32::UI::Controls::TABP_BODY,
    /// The `TABP_PANE` state.
    Pane = windows_sys::Win32::UI::Controls::TABP_PANE,
}
/// The state of the `TABP_TABITEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Tabitem {
    /// The default state.
    None = 0,
    /// The `TIS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TIS_DISABLED,
}
/// The state of the `TABP_TABITEMBOTHEDGE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Tabitembothedge {
    /// The default state.
    None = 0,
    /// The `TIBES_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TIBES_DISABLED,
}
/// The state of the `TABP_TABITEMLEFTEDGE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Tabitemleftedge {
    /// The default state.
    None = 0,
    /// The `TILES_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TILES_DISABLED,
}
/// The state of the `TABP_TABITEMRIGHTEDGE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Tabitemrightedge {
    /// The default state.
    None = 0,
    /// The `TIRES_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TIRES_DISABLED,
}
/// The state of the `TABP_TOPTABITEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Toptabitem {
    /// The default state.
    None = 0,
    /// The `TTIS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TTIS_DISABLED,
}
/// The state of the `TABP_TOPTABITEMBOTHEDGE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Toptabitembothedge {
    /// The default state.
    None = 0,
    /// The `TTIBES_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TTIBES_DISABLED,
}
/// The state of the `TABP_TOPTABITEMLEFTEDGE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Toptabitemleftedge {
    /// The default state.
    None = 0,
    /// The `TTILES_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TTILES_DISABLED,
}
/// The state of the `TABP_TOPTABITEMRIGHTEDGE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Toptabitemrightedge {
    /// The default state.
    None = 0,
    /// The `TTIRES_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TTIRES_DISABLED,
}
/// The state of the `TASKBAND` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Taskband {
    /// The default state.
    None = 0,
    /// The `TDP_GROUPCOUNT` state.
    Groupcount = windows_sys::Win32::UI::Controls::TDP_GROUPCOUNT,
    /// The `TDP_FLASHBUTTON` state.
    Flashbutton = windows_sys::Win32::UI::Controls::TDP_FLASHBUTTON,
    /// The `TDP_FLASHBUTTONGROUPMENU` state.
    Flashbuttongroupmenu = windows_sys::Win32::UI::Controls::TDP_FLASHBUTTONGROUPMENU,
}
/// The state of the `TASKBAR` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Taskbar {
    /// The default state.
    None = 0,
    /// The `TBP_BACKGROUNDBOTTOM` state.
    Backgroundbottom = windows_sys::Win32::UI::Controls::TBP_BACKGROUNDBOTTOM,
    /// The `TBP_BACKGROUNDLEFT` state.
    Backgroundleft = windows_sys::Win32::UI::Controls::TBP_BACKGROUNDLEFT,
    /// The `TBP_BACKGROUNDRIGHT` state.
    Backgroundright = windows_sys::Win32::UI::Controls::TBP_BACKGROUNDRIGHT,
    /// The `TBP_BACKGROUNDTOP` state.
    Backgroundtop = windows_sys::Win32::UI::Controls::TBP_BACKGROUNDTOP,
    /// The `TBP_SIZINGBARBOTTOM` state.
    Sizingbarbottom = windows_sys::Win32::UI::Controls::TBP_SIZINGBARBOTTOM,
    /// The `TBP_SIZINGBARBOTTOMLEFT` state.
    Sizingbarbottomleft = windows_sys::Win32::UI::Controls::TBP_SIZINGBARBOTTOMLEFT,
    /// The `TBP_SIZINGBARRIGHT` state.
    Sizingbarright = windows_sys::Win32::UI::Controls::TBP_SIZINGBARRIGHT,
    /// The `TBP_SIZINGBARTOP` state.
    Sizingbartop = windows_sys::Win32::UI::Controls::TBP_SIZINGBARTOP,
}
/// The state of the `TASKDIALOG` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Taskdialog {
    /// The default state.
    None = 0,
    /// The `TDLG_BUTTONSECTION` state.
    Buttonsection = windows_sys::Win32::UI::Controls::TDLG_BUTTONSECTION,
    /// The `TDLG_BUTTONWRAPPER` state.
    Buttonwrapper = windows_sys::Win32::UI::Controls::TDLG_BUTTONWRAPPER,
    /// The `TDLG_COMMANDLINKPANE` state.
    Commandlinkpane = windows_sys::Win32::UI::Controls::TDLG_COMMANDLINKPANE,
    /// The `TDLG_CONTENTICON` state.
    Contenticon = windows_sys::Win32::UI::Controls::TDLG_CONTENTICON,
}
/// The state of the `TDLG_CONTENTPANE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Contentpane {
    /// The default state.
    None = 0,
    /// The `TDLGCPS_STANDALONE` state.
    Standalone = windows_sys::Win32::UI::Controls::TDLGCPS_STANDALONE,
    /// The `TDLG_CONTROLPANE` state.
    Controlpane = windows_sys::Win32::UI::Controls::TDLG_CONTROLPANE,
    /// The `TDLG_EXPANDEDCONTENT` state.
    Expandedcontent = windows_sys::Win32::UI::Controls::TDLG_EXPANDEDCONTENT,
    /// The `TDLG_EXPANDEDFOOTERAREA` state.
    Expandedfooterarea = windows_sys::Win32::UI::Controls::TDLG_EXPANDEDFOOTERAREA,
}
/// The state of the `TDLG_EXPANDOBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Expandobutton {
    /// The default state.
    None = 0,
    /// The `TDLGEBS_EXPANDEDHOVER` state.
    Expandedhover = windows_sys::Win32::UI::Controls::TDLGEBS_EXPANDEDHOVER,
    /// The `TDLG_EXPANDOTEXT` state.
    Expandotext = windows_sys::Win32::UI::Controls::TDLG_EXPANDOTEXT,
    /// The `TDLG_FOOTNOTEAREA` state.
    Footnotearea = windows_sys::Win32::UI::Controls::TDLG_FOOTNOTEAREA,
    /// The `TDLG_FOOTNOTEPANE` state.
    Footnotepane = windows_sys::Win32::UI::Controls::TDLG_FOOTNOTEPANE,
    /// The `TDLG_FOOTNOTESEPARATOR` state.
    Footnoteseparator = windows_sys::Win32::UI::Controls::TDLG_FOOTNOTESEPARATOR,
    /// The `TDLG_IMAGEALIGNMENT` state.
    Imagealignment = windows_sys::Win32::UI::Controls::TDLG_IMAGEALIGNMENT,
    /// The `TDLG_MAINICON` state.
    Mainicon = windows_sys::Win32::UI::Controls::TDLG_MAINICON,
    /// The `TDLG_MAININSTRUCTIONPANE` state.
    Maininstructionpane = windows_sys::Win32::UI::Controls::TDLG_MAININSTRUCTIONPANE,
    /// The `TDLG_PRIMARYPANEL` state.
    Primarypanel = windows_sys::Win32::UI::Controls::TDLG_PRIMARYPANEL,
    /// The `TDLG_PROGRESSBAR` state.
    Progressbar = windows_sys::Win32::UI::Controls::TDLG_PROGRESSBAR,
    /// The `TDLG_RADIOBUTTONPANE` state.
    Radiobuttonpane = windows_sys::Win32::UI::Controls::TDLG_RADIOBUTTONPANE,
    /// The `TDLG_SECONDARYPANEL` state.
    Secondarypanel = windows_sys::Win32::UI::Controls::TDLG_SECONDARYPANEL,
    /// The `TDLG_VERIFICATIONTEXT` state.
    Verificationtext = windows_sys::Win32::UI::Controls::TDLG_VERIFICATIONTEXT,
}
/// The state of the `TEXTSTYLE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Textstyle {
    /// The default state.
    None = 0,
    /// The `TEXT_BODYTITLE` state.
    Bodytitle = windows_sys::Win32::UI::Controls::TEXT_BODYTITLE,
    /// The `TEXT_BODYTEXT` state.
    Bodytext = windows_sys::Win32::UI::Controls::TEXT_BODYTEXT,
}
/// The state of the `TEXT_CONTROLLABEL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Controllabel {
    /// The default state.
    None = 0,
    /// The `TS_CONTROLLABEL_DISABLED` state.
    Controllabel = windows_sys::Win32::UI::Controls::TS_CONTROLLABEL_DISABLED,
    /// The `TEXT_EXPANDED` state.
    Expanded = windows_sys::Win32::UI::Controls::TEXT_EXPANDED,
}
/// The state of the `TEXT_HYPERLINKTEXT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Hyperlinktext {
    /// The default state.
    None = 0,
    /// The `TS_HYPERLINK_DISABLED` state.
    Hyperlink = windows_sys::Win32::UI::Controls::TS_HYPERLINK_DISABLED,
    /// The `TEXT_INSTRUCTION` state.
    Instruction = windows_sys::Win32::UI::Controls::TEXT_INSTRUCTION,
    /// The `TEXT_LABEL` state.
    Label = windows_sys::Win32::UI::Controls::TEXT_LABEL,
    /// The `TEXT_MAININSTRUCTION` state.
    Maininstruction = windows_sys::Win32::UI::Controls::TEXT_MAININSTRUCTION,
    /// The `TEXT_SECONDARYTEXT` state.
    Secondarytext = windows_sys::Win32::UI::Controls::TEXT_SECONDARYTEXT,
}
/// The state of the `TP_BUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Button {
    /// The default state.
    None = 0,
    /// The `TS_CHECKED` state.
    Checked = windows_sys::Win32::UI::Controls::TS_CHECKED,
}
/// The state of the `TP_DROPDOWNBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Dropdownbutton {
    /// The default state.
    None = 0,
    /// The `TS_CHECKED` state.
    Checked = windows_sys::Win32::UI::Controls::TS_CHECKED,
}
/// The state of the `TP_DROPDOWNBUTTONGLYPH` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Dropdownbuttonglyph {
    /// The default state.
    None = 0,
    /// The `TS_CHECKED` state.
    Checked = windows_sys::Win32::UI::Controls::TS_CHECKED,
}
/// The state of the `TP_SEPARATOR` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Separator {
    /// The default state.
    None = 0,
    /// The `TS_CHECKED` state.
    Checked = windows_sys::Win32::UI::Controls::TS_CHECKED,
}
/// The state of the `TP_SEPARATORVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Separatorvert {
    /// The default state.
    None = 0,
    /// The `TS_CHECKED` state.
    Checked = windows_sys::Win32::UI::Controls::TS_CHECKED,
}
/// The state of the `TP_SPLITBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Splitbutton {
    /// The default state.
    None = 0,
    /// The `TS_CHECKED` state.
    Checked = windows_sys::Win32::UI::Controls::TS_CHECKED,
}
/// The state of the `TP_SPLITBUTTONDROPDOWN` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Splitbuttondropdown {
    /// The default state.
    None = 0,
    /// The `TS_CHECKED` state.
    Checked = windows_sys::Win32::UI::Controls::TS_CHECKED,
}
/// The state of the `TTP_BALLOON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Balloon {
    /// The default state.
    None = 0,
    /// The `TTBS_LINK` state.
    Link = windows_sys::Win32::UI::Controls::TTBS_LINK,
}
/// The state of the `TTP_BALLOONSTEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Balloonstem {
    /// The default state.
    None = 0,
    /// The `TTBSS_POINTINGUPLEFTWALL` state.
    Pointingupleftwall = windows_sys::Win32::UI::Controls::TTBSS_POINTINGUPLEFTWALL,
    /// The `TTP_BALLOONTITLE` state.
    Balloontitle = windows_sys::Win32::UI::Controls::TTP_BALLOONTITLE,
}
/// The state of the `TTP_CLOSE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Close {
    /// The default state.
    None = 0,
    /// The `TTCS_HOT` state.
    Hot = windows_sys::Win32::UI::Controls::TTCS_HOT,
}
/// The state of the `TTP_STANDARD` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Standard {
    /// The default state.
    None = 0,
    /// The `TTSS_LINK` state.
    Link = windows_sys::Win32::UI::Controls::TTSS_LINK,
}
/// The state of the `TTP_STANDARDTITLE` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Standardtitle {
    /// The default state.
    None = 0,
    /// The `TTSS_LINK` state.
    Link = windows_sys::Win32::UI::Controls::TTSS_LINK,
}
/// The state of the `TTP_WRENCH` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Wrench {
    /// The default state.
    None = 0,
    /// The `TTWS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::TTWS_NORMAL,
}
/// The state of the `TKP_THUMB` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Thumb {
    /// The default state.
    None = 0,
    /// The `TUS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TUS_DISABLED,
}
/// The state of the `TKP_THUMBBOTTOM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Thumbbottom {
    /// The default state.
    None = 0,
    /// The `TUBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TUBS_DISABLED,
}
/// The state of the `TKP_THUMBLEFT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Thumbleft {
    /// The default state.
    None = 0,
    /// The `TUVLS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TUVLS_DISABLED,
}
/// The state of the `TKP_THUMBRIGHT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Thumbright {
    /// The default state.
    None = 0,
    /// The `TUVRS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TUVRS_DISABLED,
}
/// The state of the `TKP_THUMBTOP` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Thumbtop {
    /// The default state.
    None = 0,
    /// The `TUTS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TUTS_DISABLED,
}
/// The state of the `TKP_THUMBVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Thumbvert {
    /// The default state.
    None = 0,
    /// The `TUVS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TUVS_DISABLED,
}
/// The state of the `TKP_TICS` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Tics {
    /// The default state.
    None = 0,
    /// The `TSS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::TSS_NORMAL,
}
/// The state of the `TKP_TICSVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Ticsvert {
    /// The default state.
    None = 0,
    /// The `TSVS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::TSVS_NORMAL,
}
/// The state of the `TKP_TRACK` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Track {
    /// The default state.
    None = 0,
    /// The `TRS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::TRS_NORMAL,
}
/// The state of the `TKP_TRACKVERT` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Trackvert {
    /// The default state.
    None = 0,
    /// The `TRVS_NORMAL` state.
    Normal = windows_sys::Win32::UI::Controls::TRVS_NORMAL,
}
/// The state of the `TRAYNOTIFY` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Traynotify {
    /// The default state.
    None = 0,
    /// The `TNP_ANIMBACKGROUND` state.
    Animbackground = windows_sys::Win32::UI::Controls::TNP_ANIMBACKGROUND,
    /// The `TNP_BACKGROUND` state.
    Background = windows_sys::Win32::UI::Controls::TNP_BACKGROUND,
}
/// The state of the `TREEVIEW` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Treeview {
    /// The default state.
    None = 0,
    /// The `TVP_BRANCH` state.
    Branch = windows_sys::Win32::UI::Controls::TVP_BRANCH,
}
/// The state of the `TVP_GLYPH` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Glyph {
    /// The default state.
    None = 0,
    /// The `GLPS_CLOSED` state.
    Closed = windows_sys::Win32::UI::Controls::GLPS_CLOSED,
}
/// The state of the `TVP_HOTGLYPH` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Hotglyph {
    /// The default state.
    None = 0,
    /// The `HGLPS_CLOSED` state.
    Closed = windows_sys::Win32::UI::Controls::HGLPS_CLOSED,
}
/// The state of the `TVP_TREEITEM` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Treeitem {
    /// The default state.
    None = 0,
    /// The `TREIS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::TREIS_DISABLED,
}
/// The state of the `WP_CAPTION` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Caption {
    /// The default state.
    None = 0,
    /// The `CS_ACTIVE` state.
    Active = windows_sys::Win32::UI::Controls::CS_ACTIVE,
    /// The `WP_CAPTIONSIZINGTEMPLATE` state.
    Captionsizingtemplate = windows_sys::Win32::UI::Controls::WP_CAPTIONSIZINGTEMPLATE,
}
/// The state of the `WP_CLOSEBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Closebutton {
    /// The default state.
    None = 0,
    /// The `CBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::CBS_DISABLED,
    /// The `WP_DIALOG` state.
    Dialog = windows_sys::Win32::UI::Controls::WP_DIALOG,
}
/// The state of the `WP_FRAME` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Frame {
    /// The default state.
    None = 0,
    /// The `FS_ACTIVE` state.
    Active = windows_sys::Win32::UI::Controls::FS_ACTIVE,
    /// The `WP_FRAMEBOTTOM` state.
    Framebottom = windows_sys::Win32::UI::Controls::WP_FRAMEBOTTOM,
    /// The `WP_FRAMEBOTTOMSIZINGTEMPLATE` state.
    Framebottomsizingtemplate = windows_sys::Win32::UI::Controls::WP_FRAMEBOTTOMSIZINGTEMPLATE,
    /// The `WP_FRAMELEFT` state.
    Frameleft = windows_sys::Win32::UI::Controls::WP_FRAMELEFT,
    /// The `WP_FRAMELEFTSIZINGTEMPLATE` state.
    Frameleftsizingtemplate = windows_sys::Win32::UI::Controls::WP_FRAMELEFTSIZINGTEMPLATE,
    /// The `WP_FRAMERIGHT` state.
    Frameright = windows_sys::Win32::UI::Controls::WP_FRAMERIGHT,
    /// The `WP_FRAMERIGHTSIZINGTEMPLATE` state.
    Framerightsizingtemplate = windows_sys::Win32::UI::Controls::WP_FRAMERIGHTSIZINGTEMPLATE,
}
/// The state of the `WP_HELPBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Helpbutton {
    /// The default state.
    None = 0,
    /// The `HBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::HBS_DISABLED,
}
/// The state of the `WP_HORZSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Horzscroll {
    /// The default state.
    None = 0,
    /// The `HSS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::HSS_DISABLED,
}
/// The state of the `WP_HORZTHUMB` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Horzthumb {
    /// The default state.
    None = 0,
    /// The `HTS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::HTS_DISABLED,
}
/// The state of the `WP_MAXBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Maxbutton {
    /// The default state.
    None = 0,
    /// The `MAXBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MAXBS_DISABLED,
}
/// The state of the `WP_MAXCAPTION` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Maxcaption {
    /// The default state.
    None = 0,
    /// The `MXCS_ACTIVE` state.
    Active = windows_sys::Win32::UI::Controls::MXCS_ACTIVE,
    /// The `WP_MDICLOSEBUTTON` state.
    Mdiclosebutton = windows_sys::Win32::UI::Controls::WP_MDICLOSEBUTTON,
    /// The `WP_MDIHELPBUTTON` state.
    Mdihelpbutton = windows_sys::Win32::UI::Controls::WP_MDIHELPBUTTON,
    /// The `WP_MDIMINBUTTON` state.
    Mdiminbutton = windows_sys::Win32::UI::Controls::WP_MDIMINBUTTON,
    /// The `WP_MDIRESTOREBUTTON` state.
    Mdirestorebutton = windows_sys::Win32::UI::Controls::WP_MDIRESTOREBUTTON,
    /// The `WP_MDISYSBUTTON` state.
    Mdisysbutton = windows_sys::Win32::UI::Controls::WP_MDISYSBUTTON,
}
/// The state of the `WP_MINBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Minbutton {
    /// The default state.
    None = 0,
    /// The `MINBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::MINBS_DISABLED,
}
/// The state of the `WP_MINCAPTION` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Mincaption {
    /// The default state.
    None = 0,
    /// The `MNCS_ACTIVE` state.
    Active = windows_sys::Win32::UI::Controls::MNCS_ACTIVE,
}
/// The state of the `WP_RESTOREBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Restorebutton {
    /// The default state.
    None = 0,
    /// The `RBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::RBS_DISABLED,
    /// The `WP_SMALLCAPTION` state.
    Smallcaption = windows_sys::Win32::UI::Controls::WP_SMALLCAPTION,
    /// The `WP_SMALLCAPTIONSIZINGTEMPLATE` state.
    Smallcaptionsizingtemplate = windows_sys::Win32::UI::Controls::WP_SMALLCAPTIONSIZINGTEMPLATE,
    /// The `WP_SMALLCLOSEBUTTON` state.
    Smallclosebutton = windows_sys::Win32::UI::Controls::WP_SMALLCLOSEBUTTON,
    /// The `WP_SMALLFRAMEBOTTOM` state.
    Smallframebottom = windows_sys::Win32::UI::Controls::WP_SMALLFRAMEBOTTOM,
    /// The `WP_SMALLFRAMEBOTTOMSIZINGTEMPLATE` state.
    Smallframebottomsizingtemplate = windows_sys::Win32::UI::Controls::WP_SMALLFRAMEBOTTOMSIZINGTEMPLATE,
    /// The `WP_SMALLFRAMELEFT` state.
    Smallframeleft = windows_sys::Win32::UI::Controls::WP_SMALLFRAMELEFT,
    /// The `WP_SMALLFRAMELEFTSIZINGTEMPLATE` state.
    Smallframeleftsizingtemplate = windows_sys::Win32::UI::Controls::WP_SMALLFRAMELEFTSIZINGTEMPLATE,
    /// The `WP_SMALLFRAMERIGHT` state.
    Smallframeright = windows_sys::Win32::UI::Controls::WP_SMALLFRAMERIGHT,
    /// The `WP_SMALLFRAMERIGHTSIZINGTEMPLATE` state.
    Smallframerightsizingtemplate = windows_sys::Win32::UI::Controls::WP_SMALLFRAMERIGHTSIZINGTEMPLATE,
    /// The `WP_SMALLMAXCAPTION` state.
    Smallmaxcaption = windows_sys::Win32::UI::Controls::WP_SMALLMAXCAPTION,
    /// The `WP_SMALLMINCAPTION` state.
    Smallmincaption = windows_sys::Win32::UI::Controls::WP_SMALLMINCAPTION,
}
/// The state of the `WP_SYSBUTTON` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Sysbutton {
    /// The default state.
    None = 0,
    /// The `SBS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::SBS_DISABLED,
}
/// The state of the `WP_VERTSCROLL` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Vertscroll {
    /// The default state.
    None = 0,
    /// The `VSS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::VSS_DISABLED,
}
/// The state of the `WP_VERTTHUMB` part.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(i32)]
pub enum Vertthumb {
    /// The default state.
    None = 0,
    /// The `VTS_DISABLED` state.
    Disabled = windows_sys::Win32::UI::Controls::VTS_DISABLED,
}
impl Part {
    /// Returns the part and state for this part.
    pub(super) fn part_and_state(self) -> (i32, i32) {
        match self {
            Self::Checkbox(state) => (windows_sys::Win32::UI::Controls::BP_CHECKBOX, state as i32),
            Self::Commandlink(state) => (windows_sys::Win32::UI::Controls::BP_COMMANDLINK, state as i32),
            Self::Commandlinkglyph(state) => (windows_sys::Win32::UI::Controls::BP_COMMANDLINKGLYPH, state as i32),
            Self::Groupbox(state) => (windows_sys::Win32::UI::Controls::BP_GROUPBOX, state as i32),
            Self::Pushbutton(state) => (windows_sys::Win32::UI::Controls::BP_PUSHBUTTON, state as i32),
            Self::Radiobutton(state) => (windows_sys::Win32::UI::Controls::BP_RADIOBUTTON, state as i32),
            Self::Time(state) => (windows_sys::Win32::UI::Controls::CLP_TIME, state as i32),
            Self::Combobox(state) => (windows_sys::Win32::UI::Controls::COMBOBOX, state as i32),
            Self::Border(state) => (windows_sys::Win32::UI::Controls::CP_BORDER, state as i32),
            Self::Cuebanner(state) => (windows_sys::Win32::UI::Controls::CP_CUEBANNER, state as i32),
            Self::Dropdownbutton(state) => (windows_sys::Win32::UI::Controls::CP_DROPDOWNBUTTON, state as i32),
            Self::Dropdownbuttonleft(state) => (windows_sys::Win32::UI::Controls::CP_DROPDOWNBUTTONLEFT, state as i32),
            Self::Dropdownbuttonright(state) => (windows_sys::Win32::UI::Controls::CP_DROPDOWNBUTTONRIGHT, state as i32),
            Self::Transparentbackground(state) => (windows_sys::Win32::UI::Controls::CP_TRANSPARENTBACKGROUND, state as i32),
            Self::Readonly(state) => (windows_sys::Win32::UI::Controls::CP_READONLY, state as i32),
            Self::Tab(state) => (windows_sys::Win32::UI::Controls::CSST_TAB, state as i32),
            Self::Controlpanel(state) => (windows_sys::Win32::UI::Controls::CONTROLPANEL, state as i32),
            Self::Contentlink(state) => (windows_sys::Win32::UI::Controls::CPANEL_CONTENTLINK, state as i32),
            Self::Helplink(state) => (windows_sys::Win32::UI::Controls::CPANEL_HELPLINK, state as i32),
            Self::Sectiontitlelink(state) => (windows_sys::Win32::UI::Controls::CPANEL_SECTIONTITLELINK, state as i32),
            Self::Tasklink(state) => (windows_sys::Win32::UI::Controls::CPANEL_TASKLINK, state as i32),
            Self::Dateborder(state) => (windows_sys::Win32::UI::Controls::DP_DATEBORDER, state as i32),
            Self::Datetext(state) => (windows_sys::Win32::UI::Controls::DP_DATETEXT, state as i32),
            Self::Showcalendarbuttonright(state) => (windows_sys::Win32::UI::Controls::DP_SHOWCALENDARBUTTONRIGHT, state as i32),
            Self::Copy(state) => (windows_sys::Win32::UI::Controls::DD_COPY, state as i32),
            Self::Createlink(state) => (windows_sys::Win32::UI::Controls::DD_CREATELINK, state as i32),
            Self::Move(state) => (windows_sys::Win32::UI::Controls::DD_MOVE, state as i32),
            Self::None(state) => (windows_sys::Win32::UI::Controls::DD_NONE, state as i32),
            Self::Updatemetadata(state) => (windows_sys::Win32::UI::Controls::DD_UPDATEMETADATA, state as i32),
            Self::Warning(state) => (windows_sys::Win32::UI::Controls::DD_WARNING, state as i32),
            Self::Background(state) => (windows_sys::Win32::UI::Controls::EP_BACKGROUND, state as i32),
            Self::Backgroundwithborder(state) => (windows_sys::Win32::UI::Controls::EP_BACKGROUNDWITHBORDER, state as i32),
            Self::Editborder(state) => (windows_sys::Win32::UI::Controls::EP_EDITBORDER_HSCROLL, state as i32),
            Self::Editborder(state) => (windows_sys::Win32::UI::Controls::EP_EDITBORDER_HVSCROLL, state as i32),
            Self::Editborder(state) => (windows_sys::Win32::UI::Controls::EP_EDITBORDER_NOSCROLL, state as i32),
            Self::Editborder(state) => (windows_sys::Win32::UI::Controls::EP_EDITBORDER_VSCROLL, state as i32),
            Self::Edittext(state) => (windows_sys::Win32::UI::Controls::EP_EDITTEXT, state as i32),
            Self::Explorerbar(state) => (windows_sys::Win32::UI::Controls::EXPLORERBAR, state as i32),
            Self::Headerclose(state) => (windows_sys::Win32::UI::Controls::EBP_HEADERCLOSE, state as i32),
            Self::Headerpin(state) => (windows_sys::Win32::UI::Controls::EBP_HEADERPIN, state as i32),
            Self::Iebarmenu(state) => (windows_sys::Win32::UI::Controls::EBP_IEBARMENU, state as i32),
            Self::Normalgroupcollapse(state) => (windows_sys::Win32::UI::Controls::EBP_NORMALGROUPCOLLAPSE, state as i32),
            Self::Normalgroupexpand(state) => (windows_sys::Win32::UI::Controls::EBP_NORMALGROUPEXPAND, state as i32),
            Self::Specialgroupcollapse(state) => (windows_sys::Win32::UI::Controls::EBP_SPECIALGROUPCOLLAPSE, state as i32),
            Self::Specialgroupexpand(state) => (windows_sys::Win32::UI::Controls::EBP_SPECIALGROUPEXPAND, state as i32),
            Self::Body(state) => (windows_sys::Win32::UI::Controls::FLYOUT_BODY, state as i32),
            Self::Label(state) => (windows_sys::Win32::UI::Controls::FLYOUT_LABEL, state as i32),
            Self::Link(state) => (windows_sys::Win32::UI::Controls::FLYOUT_LINK, state as i32),
            Self::Linkheader(state) => (windows_sys::Win32::UI::Controls::FLYOUT_LINKHEADER, state as i32),
            Self::Border(state) => (windows_sys::Win32::UI::Controls::GP_BORDER, state as i32),
            Self::Linehorz(state) => (windows_sys::Win32::UI::Controls::GP_LINEHORZ, state as i32),
            Self::Linevert(state) => (windows_sys::Win32::UI::Controls::GP_LINEVERT, state as i32),
            Self::Headerdropdown(state) => (windows_sys::Win32::UI::Controls::HP_HEADERDROPDOWN, state as i32),
            Self::Headerdropdownfilter(state) => (windows_sys::Win32::UI::Controls::HP_HEADERDROPDOWNFILTER, state as i32),
            Self::Headeritem(state) => (windows_sys::Win32::UI::Controls::HP_HEADERITEM, state as i32),
            Self::Headeritemleft(state) => (windows_sys::Win32::UI::Controls::HP_HEADERITEMLEFT, state as i32),
            Self::Headeritemright(state) => (windows_sys::Win32::UI::Controls::HP_HEADERITEMRIGHT, state as i32),
            Self::Headeroverflow(state) => (windows_sys::Win32::UI::Controls::HP_HEADEROVERFLOW, state as i32),
            Self::Headersortarrow(state) => (windows_sys::Win32::UI::Controls::HP_HEADERSORTARROW, state as i32),
            Self::Border(state) => (windows_sys::Win32::UI::Controls::LBCP_BORDER_HSCROLL, state as i32),
            Self::Border(state) => (windows_sys::Win32::UI::Controls::LBCP_BORDER_HVSCROLL, state as i32),
            Self::Border(state) => (windows_sys::Win32::UI::Controls::LBCP_BORDER_NOSCROLL, state as i32),
            Self::Border(state) => (windows_sys::Win32::UI::Controls::LBCP_BORDER_VSCROLL, state as i32),
            Self::Item(state) => (windows_sys::Win32::UI::Controls::LBCP_ITEM, state as i32),
            Self::Collapsebutton(state) => (windows_sys::Win32::UI::Controls::LVP_COLLAPSEBUTTON, state as i32),
            Self::Expandbutton(state) => (windows_sys::Win32::UI::Controls::LVP_EXPANDBUTTON, state as i32),
            Self::Groupheader(state) => (windows_sys::Win32::UI::Controls::LVP_GROUPHEADER, state as i32),
            Self::Groupheaderline(state) => (windows_sys::Win32::UI::Controls::LVP_GROUPHEADERLINE, state as i32),
            Self::Listitem(state) => (windows_sys::Win32::UI::Controls::LVP_LISTITEM, state as i32),
            Self::Barbackground(state) => (windows_sys::Win32::UI::Controls::MENU_BARBACKGROUND, state as i32),
            Self::Baritem(state) => (windows_sys::Win32::UI::Controls::MENU_BARITEM, state as i32),
            Self::Popupcheck(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPCHECK, state as i32),
            Self::Popupcheckbackground(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPCHECKBACKGROUND, state as i32),
            Self::Popupitem(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPITEM, state as i32),
            Self::Popupitem(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPITEM_FOCUSABLE, state as i32),
            Self::Popupitemkbfocus(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPITEMKBFOCUS, state as i32),
            Self::Popupsubmenu(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPSUBMENU, state as i32),
            Self::Systemclose(state) => (windows_sys::Win32::UI::Controls::MENU_SYSTEMCLOSE, state as i32),
            Self::Systemmaximize(state) => (windows_sys::Win32::UI::Controls::MENU_SYSTEMMAXIMIZE, state as i32),
            Self::Systemminimize(state) => (windows_sys::Win32::UI::Controls::MENU_SYSTEMMINIMIZE, state as i32),
            Self::Systemrestore(state) => (windows_sys::Win32::UI::Controls::MENU_SYSTEMRESTORE, state as i32),
            Self::Newappbutton(state) => (windows_sys::Win32::UI::Controls::MDP_NEWAPPBUTTON, state as i32),
            Self::Backbutton(state) => (windows_sys::Win32::UI::Controls::NAV_BACKBUTTON, state as i32),
            Self::Forwardbutton(state) => (windows_sys::Win32::UI::Controls::NAV_FORWARDBUTTON, state as i32),
            Self::Menubutton(state) => (windows_sys::Win32::UI::Controls::NAV_MENUBUTTON, state as i32),
            Self::Down(state) => (windows_sys::Win32::UI::Controls::PGRP_DOWN, state as i32),
            Self::Downhorz(state) => (windows_sys::Win32::UI::Controls::PGRP_DOWNHORZ, state as i32),
            Self::Up(state) => (windows_sys::Win32::UI::Controls::PGRP_UP, state as i32),
            Self::Uphorz(state) => (windows_sys::Win32::UI::Controls::PGRP_UPHORZ, state as i32),
            Self::Progress(state) => (windows_sys::Win32::UI::Controls::PROGRESS, state as i32),
            Self::Fill(state) => (windows_sys::Win32::UI::Controls::PP_FILL, state as i32),
            Self::Fillvert(state) => (windows_sys::Win32::UI::Controls::PP_FILLVERT, state as i32),
            Self::Transparentbar(state) => (windows_sys::Win32::UI::Controls::PP_TRANSPARENTBAR, state as i32),
            Self::Transparentbarvert(state) => (windows_sys::Win32::UI::Controls::PP_TRANSPARENTBARVERT, state as i32),
            Self::Rebar(state) => (windows_sys::Win32::UI::Controls::REBAR, state as i32),
            Self::Chevron(state) => (windows_sys::Win32::UI::Controls::RP_CHEVRON, state as i32),
            Self::Chevronvert(state) => (windows_sys::Win32::UI::Controls::RP_CHEVRONVERT, state as i32),
            Self::Splitter(state) => (windows_sys::Win32::UI::Controls::RP_SPLITTER, state as i32),
            Self::Splittervert(state) => (windows_sys::Win32::UI::Controls::RP_SPLITTERVERT, state as i32),
            Self::Arrowbtn(state) => (windows_sys::Win32::UI::Controls::SBP_ARROWBTN, state as i32),
            Self::Gripperhorz(state) => (windows_sys::Win32::UI::Controls::SBP_GRIPPERHORZ, state as i32),
            Self::Grippervert(state) => (windows_sys::Win32::UI::Controls::SBP_GRIPPERVERT, state as i32),
            Self::Lowertrackhorz(state) => (windows_sys::Win32::UI::Controls::SBP_LOWERTRACKHORZ, state as i32),
            Self::Lowertrackvert(state) => (windows_sys::Win32::UI::Controls::SBP_LOWERTRACKVERT, state as i32),
            Self::Sizebox(state) => (windows_sys::Win32::UI::Controls::SBP_SIZEBOX, state as i32),
            Self::Thumbbtnhorz(state) => (windows_sys::Win32::UI::Controls::SBP_THUMBBTNHORZ, state as i32),
            Self::Thumbbtnvert(state) => (windows_sys::Win32::UI::Controls::SBP_THUMBBTNVERT, state as i32),
            Self::Uppertrackhorz(state) => (windows_sys::Win32::UI::Controls::SBP_UPPERTRACKHORZ, state as i32),
            Self::Uppertrackvert(state) => (windows_sys::Win32::UI::Controls::SBP_UPPERTRACKVERT, state as i32),
            Self::Searcheditboxtext(state) => (windows_sys::Win32::UI::Controls::SEBP_SEARCHEDITBOXTEXT, state as i32),
            Self::Down(state) => (windows_sys::Win32::UI::Controls::SPNP_DOWN, state as i32),
            Self::Downhorz(state) => (windows_sys::Win32::UI::Controls::SPNP_DOWNHORZ, state as i32),
            Self::Up(state) => (windows_sys::Win32::UI::Controls::SPNP_UP, state as i32),
            Self::Uphorz(state) => (windows_sys::Win32::UI::Controls::SPNP_UPHORZ, state as i32),
            Self::Startpanel(state) => (windows_sys::Win32::UI::Controls::STARTPANEL, state as i32),
            Self::Logoffbuttons(state) => (windows_sys::Win32::UI::Controls::SPP_LOGOFFBUTTONS, state as i32),
            Self::Moreprogramsarrow(state) => (windows_sys::Win32::UI::Controls::SPP_MOREPROGRAMSARROW, state as i32),
            Self::Status(state) => (windows_sys::Win32::UI::Controls::STATUS, state as i32),
            Self::Tab(state) => (windows_sys::Win32::UI::Controls::TAB, state as i32),
            Self::Tabitem(state) => (windows_sys::Win32::UI::Controls::TABP_TABITEM, state as i32),
            Self::Tabitembothedge(state) => (windows_sys::Win32::UI::Controls::TABP_TABITEMBOTHEDGE, state as i32),
            Self::Tabitemleftedge(state) => (windows_sys::Win32::UI::Controls::TABP_TABITEMLEFTEDGE, state as i32),
            Self::Tabitemrightedge(state) => (windows_sys::Win32::UI::Controls::TABP_TABITEMRIGHTEDGE, state as i32),
            Self::Toptabitem(state) => (windows_sys::Win32::UI::Controls::TABP_TOPTABITEM, state as i32),
            Self::Toptabitembothedge(state) => (windows_sys::Win32::UI::Controls::TABP_TOPTABITEMBOTHEDGE, state as i32),
            Self::Toptabitemleftedge(state) => (windows_sys::Win32::UI::Controls::TABP_TOPTABITEMLEFTEDGE, state as i32),
            Self::Toptabitemrightedge(state) => (windows_sys::Win32::UI::Controls::TABP_TOPTABITEMRIGHTEDGE, state as i32),
            Self::Taskband(state) => (windows_sys::Win32::UI::Controls::TASKBAND, state as i32),
            Self::Taskbar(state) => (windows_sys::Win32::UI::Controls::TASKBAR, state as i32),
            Self::Taskdialog(state) => (windows_sys::Win32::UI::Controls::TASKDIALOG, state as i32),
            Self::Contentpane(state) => (windows_sys::Win32::UI::Controls::TDLG_CONTENTPANE, state as i32),
            Self::Expandobutton(state) => (windows_sys::Win32::UI::Controls::TDLG_EXPANDOBUTTON, state as i32),
            Self::Textstyle(state) => (windows_sys::Win32::UI::Controls::TEXTSTYLE, state as i32),
            Self::Controllabel(state) => (windows_sys::Win32::UI::Controls::TEXT_CONTROLLABEL, state as i32),
            Self::Hyperlinktext(state) => (windows_sys::Win32::UI::Controls::TEXT_HYPERLINKTEXT, state as i32),
            Self::Button(state) => (windows_sys::Win32::UI::Controls::TP_BUTTON, state as i32),
            Self::Dropdownbutton(state) => (windows_sys::Win32::UI::Controls::TP_DROPDOWNBUTTON, state as i32),
            Self::Dropdownbuttonglyph(state) => (windows_sys::Win32::UI::Controls::TP_DROPDOWNBUTTONGLYPH, state as i32),
            Self::Separator(state) => (windows_sys::Win32::UI::Controls::TP_SEPARATOR, state as i32),
            Self::Separatorvert(state) => (windows_sys::Win32::UI::Controls::TP_SEPARATORVERT, state as i32),
            Self::Splitbutton(state) => (windows_sys::Win32::UI::Controls::TP_SPLITBUTTON, state as i32),
            Self::Splitbuttondropdown(state) => (windows_sys::Win32::UI::Controls::TP_SPLITBUTTONDROPDOWN, state as i32),
            Self::Balloon(state) => (windows_sys::Win32::UI::Controls::TTP_BALLOON, state as i32),
            Self::Balloonstem(state) => (windows_sys::Win32::UI::Controls::TTP_BALLOONSTEM, state as i32),
            Self::Close(state) => (windows_sys::Win32::UI::Controls::TTP_CLOSE, state as i32),
            Self::Standard(state) => (windows_sys::Win32::UI::Controls::TTP_STANDARD, state as i32),
            Self::Standardtitle(state) => (windows_sys::Win32::UI::Controls::TTP_STANDARDTITLE, state as i32),
            Self::Wrench(state) => (windows_sys::Win32::UI::Controls::TTP_WRENCH, state as i32),
            Self::Thumb(state) => (windows_sys::Win32::UI::Controls::TKP_THUMB, state as i32),
            Self::Thumbbottom(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBBOTTOM, state as i32),
            Self::Thumbleft(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBLEFT, state as i32),
            Self::Thumbright(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBRIGHT, state as i32),
            Self::Thumbtop(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBTOP, state as i32),
            Self::Thumbvert(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBVERT, state as i32),
            Self::Tics(state) => (windows_sys::Win32::UI::Controls::TKP_TICS, state as i32),
            Self::Ticsvert(state) => (windows_sys::Win32::UI::Controls::TKP_TICSVERT, state as i32),
            Self::Track(state) => (windows_sys::Win32::UI::Controls::TKP_TRACK, state as i32),
            Self::Trackvert(state) => (windows_sys::Win32::UI::Controls::TKP_TRACKVERT, state as i32),
            Self::Traynotify(state) => (windows_sys::Win32::UI::Controls::TRAYNOTIFY, state as i32),
            Self::Treeview(state) => (windows_sys::Win32::UI::Controls::TREEVIEW, state as i32),
            Self::Glyph(state) => (windows_sys::Win32::UI::Controls::TVP_GLYPH, state as i32),
            Self::Hotglyph(state) => (windows_sys::Win32::UI::Controls::TVP_HOTGLYPH, state as i32),
            Self::Treeitem(state) => (windows_sys::Win32::UI::Controls::TVP_TREEITEM, state as i32),
            Self::Caption(state) => (windows_sys::Win32::UI::Controls::WP_CAPTION, state as i32),
            Self::Closebutton(state) => (windows_sys::Win32::UI::Controls::WP_CLOSEBUTTON, state as i32),
            Self::Frame(state) => (windows_sys::Win32::UI::Controls::WP_FRAME, state as i32),
            Self::Helpbutton(state) => (windows_sys::Win32::UI::Controls::WP_HELPBUTTON, state as i32),
            Self::Horzscroll(state) => (windows_sys::Win32::UI::Controls::WP_HORZSCROLL, state as i32),
            Self::Horzthumb(state) => (windows_sys::Win32::UI::Controls::WP_HORZTHUMB, state as i32),
            Self::Maxbutton(state) => (windows_sys::Win32::UI::Controls::WP_MAXBUTTON, state as i32),
            Self::Maxcaption(state) => (windows_sys::Win32::UI::Controls::WP_MAXCAPTION, state as i32),
            Self::Minbutton(state) => (windows_sys::Win32::UI::Controls::WP_MINBUTTON, state as i32),
            Self::Mincaption(state) => (windows_sys::Win32::UI::Controls::WP_MINCAPTION, state as i32),
            Self::Restorebutton(state) => (windows_sys::Win32::UI::Controls::WP_RESTOREBUTTON, state as i32),
            Self::Sysbutton(state) => (windows_sys::Win32::UI::Controls::WP_SYSBUTTON, state as i32),
            Self::Vertscroll(state) => (windows_sys::Win32::UI::Controls::WP_VERTSCROLL, state as i32),
            Self::Vertthumb(state) => (windows_sys::Win32::UI::Controls::WP_VERTTHUMB, state as i32),
        }
    }
}
