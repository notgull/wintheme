// SPDX-License-Identifier: BSL-1.0 OR Apache-2.0
// Automatically generated, do not manually edit.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

/// The part of the widget theme to retrieve.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Part {
    /// The `BP_CHECKBOX` part.
    BpCheckbox(BpCheckbox),
    /// The `BP_COMMANDLINK` part.
    BpCommandlink(BpCommandlink),
    /// The `BP_COMMANDLINKGLYPH` part.
    BpCommandlinkglyph(BpCommandlinkglyph),
    /// The `BP_GROUPBOX` part.
    BpGroupbox(BpGroupbox),
    /// The `BP_PUSHBUTTON` part.
    BpPushbutton(BpPushbutton),
    /// The `BP_RADIOBUTTON` part.
    BpRadiobutton(BpRadiobutton),
    /// The `BP_USERBUTTON` part.
    BpUserbutton(BpUserbutton),
    /// The `CLP_TIME` part.
    ClpTime(ClpTime),
    /// The `CP_BACKGROUND` part.
    CpBackground(CpBackground),
    /// The `CP_BORDER` part.
    CpBorder(CpBorder),
    /// The `CP_CUEBANNER` part.
    CpCuebanner(CpCuebanner),
    /// The `CP_DROPDOWNBUTTON` part.
    CpDropdownbutton(CpDropdownbutton),
    /// The `CP_DROPDOWNBUTTONLEFT` part.
    CpDropdownbuttonleft(CpDropdownbuttonleft),
    /// The `CP_DROPDOWNBUTTONRIGHT` part.
    CpDropdownbuttonright(CpDropdownbuttonright),
    /// The `CP_TRANSPARENTBACKGROUND` part.
    CpTransparentbackground(CpTransparentbackground),
    /// The `CP_READONLY` part.
    CpReadonly(CpReadonly),
    /// The `CSST_TAB` part.
    CsstTab(CsstTab),
    /// The `CPANEL_BANNERAREA` part.
    CpanelBannerarea(CpanelBannerarea),
    /// The `CPANEL_BODYTEXT` part.
    CpanelBodytext(CpanelBodytext),
    /// The `CPANEL_BODYTITLE` part.
    CpanelBodytitle(CpanelBodytitle),
    /// The `CPANEL_BUTTON` part.
    CpanelButton(CpanelButton),
    /// The `CPANEL_CONTENTLINK` part.
    CpanelContentlink(CpanelContentlink),
    /// The `CPANEL_CONTENTPANE` part.
    CpanelContentpane(CpanelContentpane),
    /// The `CPANEL_CONTENTPANELABEL` part.
    CpanelContentpanelabel(CpanelContentpanelabel),
    /// The `CPANEL_CONTENTPANELINE` part.
    CpanelContentpaneline(CpanelContentpaneline),
    /// The `CPANEL_GROUPTEXT` part.
    CpanelGrouptext(CpanelGrouptext),
    /// The `CPANEL_HELPLINK` part.
    CpanelHelplink(CpanelHelplink),
    /// The `CPANEL_LARGECOMMANDAREA` part.
    CpanelLargecommandarea(CpanelLargecommandarea),
    /// The `CPANEL_MESSAGETEXT` part.
    CpanelMessagetext(CpanelMessagetext),
    /// The `CPANEL_NAVIGATIONPANE` part.
    CpanelNavigationpane(CpanelNavigationpane),
    /// The `CPANEL_NAVIGATIONPANELABEL` part.
    CpanelNavigationpanelabel(CpanelNavigationpanelabel),
    /// The `CPANEL_NAVIGATIONPANELINE` part.
    CpanelNavigationpaneline(CpanelNavigationpaneline),
    /// The `CPANEL_SECTIONTITLELINK` part.
    CpanelSectiontitlelink(CpanelSectiontitlelink),
    /// The `CPANEL_SMALLCOMMANDAREA` part.
    CpanelSmallcommandarea(CpanelSmallcommandarea),
    /// The `CPANEL_TASKLINK` part.
    CpanelTasklink(CpanelTasklink),
    /// The `CPANEL_TITLE` part.
    CpanelTitle(CpanelTitle),
    /// The `DP_DATEBORDER` part.
    DpDateborder(DpDateborder),
    /// The `DP_DATETEXT` part.
    DpDatetext(DpDatetext),
    /// The `DP_SHOWCALENDARBUTTONRIGHT` part.
    DpShowcalendarbuttonright(DpShowcalendarbuttonright),
    /// The `DD_COPY` part.
    DdCopy(DdCopy),
    /// The `DD_CREATELINK` part.
    DdCreatelink(DdCreatelink),
    /// The `DD_IMAGEBG` part.
    DdImagebg(DdImagebg),
    /// The `DD_MOVE` part.
    DdMove(DdMove),
    /// The `DD_NONE` part.
    DdNone(DdNone),
    /// The `DD_TEXTBG` part.
    DdTextbg(DdTextbg),
    /// The `DD_UPDATEMETADATA` part.
    DdUpdatemetadata(DdUpdatemetadata),
    /// The `DD_WARNING` part.
    DdWarning(DdWarning),
    /// The `EP_BACKGROUND` part.
    EpBackground(EpBackground),
    /// The `EP_BACKGROUNDWITHBORDER` part.
    EpBackgroundwithborder(EpBackgroundwithborder),
    /// The `EP_CARET` part.
    EpCaret(EpCaret),
    /// The `EP_EDITBORDER_HSCROLL` part.
    EpEditborderHscroll(EpEditborderHscroll),
    /// The `EP_EDITBORDER_HVSCROLL` part.
    EpEditborderHvscroll(EpEditborderHvscroll),
    /// The `EP_EDITBORDER_NOSCROLL` part.
    EpEditborderNoscroll(EpEditborderNoscroll),
    /// The `EP_EDITBORDER_VSCROLL` part.
    EpEditborderVscroll(EpEditborderVscroll),
    /// The `EP_EDITTEXT` part.
    EpEdittext(EpEdittext),
    /// The `EP_PASSWORD` part.
    EpPassword(EpPassword),
    /// The `EBP_HEADERBACKGROUND` part.
    EbpHeaderbackground(EbpHeaderbackground),
    /// The `EBP_HEADERCLOSE` part.
    EbpHeaderclose(EbpHeaderclose),
    /// The `EBP_HEADERPIN` part.
    EbpHeaderpin(EbpHeaderpin),
    /// The `EBP_IEBARMENU` part.
    EbpIebarmenu(EbpIebarmenu),
    /// The `EBP_NORMALGROUPBACKGROUND` part.
    EbpNormalgroupbackground(EbpNormalgroupbackground),
    /// The `EBP_NORMALGROUPCOLLAPSE` part.
    EbpNormalgroupcollapse(EbpNormalgroupcollapse),
    /// The `EBP_NORMALGROUPEXPAND` part.
    EbpNormalgroupexpand(EbpNormalgroupexpand),
    /// The `EBP_NORMALGROUPHEAD` part.
    EbpNormalgrouphead(EbpNormalgrouphead),
    /// The `EBP_SPECIALGROUPBACKGROUND` part.
    EbpSpecialgroupbackground(EbpSpecialgroupbackground),
    /// The `EBP_SPECIALGROUPCOLLAPSE` part.
    EbpSpecialgroupcollapse(EbpSpecialgroupcollapse),
    /// The `EBP_SPECIALGROUPEXPAND` part.
    EbpSpecialgroupexpand(EbpSpecialgroupexpand),
    /// The `EBP_SPECIALGROUPHEAD` part.
    EbpSpecialgrouphead(EbpSpecialgrouphead),
    /// The `FLYOUT_BODY` part.
    FlyoutBody(FlyoutBody),
    /// The `FLYOUT_DIVIDER` part.
    FlyoutDivider(FlyoutDivider),
    /// The `FLYOUT_HEADER` part.
    FlyoutHeader(FlyoutHeader),
    /// The `FLYOUT_LABEL` part.
    FlyoutLabel(FlyoutLabel),
    /// The `FLYOUT_LINK` part.
    FlyoutLink(FlyoutLink),
    /// The `FLYOUT_LINKAREA` part.
    FlyoutLinkarea(FlyoutLinkarea),
    /// The `FLYOUT_LINKHEADER` part.
    FlyoutLinkheader(FlyoutLinkheader),
    /// The `FLYOUT_WINDOW` part.
    FlyoutWindow(FlyoutWindow),
    /// The `HP_HEADERDROPDOWN` part.
    HpHeaderdropdown(HpHeaderdropdown),
    /// The `HP_HEADERDROPDOWNFILTER` part.
    HpHeaderdropdownfilter(HpHeaderdropdownfilter),
    /// The `HP_HEADERITEM` part.
    HpHeaderitem(HpHeaderitem),
    /// The `HP_HEADERITEMLEFT` part.
    HpHeaderitemleft(HpHeaderitemleft),
    /// The `HP_HEADERITEMRIGHT` part.
    HpHeaderitemright(HpHeaderitemright),
    /// The `HP_HEADEROVERFLOW` part.
    HpHeaderoverflow(HpHeaderoverflow),
    /// The `HP_HEADERSORTARROW` part.
    HpHeadersortarrow(HpHeadersortarrow),
    /// The `LBCP_BORDER_HSCROLL` part.
    LbcpBorderHscroll(LbcpBorderHscroll),
    /// The `LBCP_BORDER_HVSCROLL` part.
    LbcpBorderHvscroll(LbcpBorderHvscroll),
    /// The `LBCP_BORDER_NOSCROLL` part.
    LbcpBorderNoscroll(LbcpBorderNoscroll),
    /// The `LBCP_BORDER_VSCROLL` part.
    LbcpBorderVscroll(LbcpBorderVscroll),
    /// The `LBCP_ITEM` part.
    LbcpItem(LbcpItem),
    /// The `LVP_COLLAPSEBUTTON` part.
    LvpCollapsebutton(LvpCollapsebutton),
    /// The `LVP_COLUMNDETAIL` part.
    LvpColumndetail(LvpColumndetail),
    /// The `LVP_EMPTYTEXT` part.
    LvpEmptytext(LvpEmptytext),
    /// The `LVP_EXPANDBUTTON` part.
    LvpExpandbutton(LvpExpandbutton),
    /// The `LVP_GROUPHEADER` part.
    LvpGroupheader(LvpGroupheader),
    /// The `LVP_GROUPHEADERLINE` part.
    LvpGroupheaderline(LvpGroupheaderline),
    /// The `LVP_LISTGROUP` part.
    LvpListgroup(LvpListgroup),
    /// The `LVP_LISTDETAIL` part.
    LvpListdetail(LvpListdetail),
    /// The `LVP_LISTITEM` part.
    LvpListitem(LvpListitem),
    /// The `LVP_LISTSORTEDDETAIL` part.
    LvpListsorteddetail(LvpListsorteddetail),
    /// The `MENU_BARBACKGROUND` part.
    MenuBarbackground(MenuBarbackground),
    /// The `MENU_BARITEM` part.
    MenuBaritem(MenuBaritem),
    /// The `MENU_CHEVRON_TMSCHEMA` part.
    MenuChevronTmschema(MenuChevronTmschema),
    /// The `MENU_MENUBARDROPDOWN_TMSCHEMA` part.
    MenuMenubardropdownTmschema(MenuMenubardropdownTmschema),
    /// The `MENU_MENUBARITEM_TMSCHEMA` part.
    MenuMenubaritemTmschema(MenuMenubaritemTmschema),
    /// The `MENU_MENUDROPDOWN_TMSCHEMA` part.
    MenuMenudropdownTmschema(MenuMenudropdownTmschema),
    /// The `MENU_MENUITEM_TMSCHEMA` part.
    MenuMenuitemTmschema(MenuMenuitemTmschema),
    /// The `MENU_POPUPBACKGROUND` part.
    MenuPopupbackground(MenuPopupbackground),
    /// The `MENU_POPUPBORDERS` part.
    MenuPopupborders(MenuPopupborders),
    /// The `MENU_POPUPCHECK` part.
    MenuPopupcheck(MenuPopupcheck),
    /// The `MENU_POPUPCHECKBACKGROUND` part.
    MenuPopupcheckbackground(MenuPopupcheckbackground),
    /// The `MENU_POPUPGUTTER` part.
    MenuPopupgutter(MenuPopupgutter),
    /// The `MENU_POPUPITEM` part.
    MenuPopupitem(MenuPopupitem),
    /// The `MENU_POPUPSEPARATOR` part.
    MenuPopupseparator(MenuPopupseparator),
    /// The `MENU_POPUPSUBMENU` part.
    MenuPopupsubmenu(MenuPopupsubmenu),
    /// The `MENU_SEPARATOR_TMSCHEMA` part.
    MenuSeparatorTmschema(MenuSeparatorTmschema),
    /// The `MENU_SYSTEMCLOSE` part.
    MenuSystemclose(MenuSystemclose),
    /// The `MENU_SYSTEMMAXIMIZE` part.
    MenuSystemmaximize(MenuSystemmaximize),
    /// The `MENU_SYSTEMMINIMIZE` part.
    MenuSystemminimize(MenuSystemminimize),
    /// The `MENU_SYSTEMRESTORE` part.
    MenuSystemrestore(MenuSystemrestore),
    /// The `MDP_NEWAPPBUTTON` part.
    MdpNewappbutton(MdpNewappbutton),
    /// The `MDP_SEPERATOR` part.
    MdpSeperator(MdpSeperator),
    /// The `NAV_BACKBUTTON` part.
    NavBackbutton(NavBackbutton),
    /// The `NAV_FORWARDBUTTON` part.
    NavForwardbutton(NavForwardbutton),
    /// The `NAV_MENUBUTTON` part.
    NavMenubutton(NavMenubutton),
    /// The `PGRP_DOWN` part.
    PgrpDown(PgrpDown),
    /// The `PGRP_DOWNHORZ` part.
    PgrpDownhorz(PgrpDownhorz),
    /// The `PGRP_UP` part.
    PgrpUp(PgrpUp),
    /// The `PGRP_UPHORZ` part.
    PgrpUphorz(PgrpUphorz),
    /// The `PP_BAR` part.
    PpBar(PpBar),
    /// The `PP_BARVERT` part.
    PpBarvert(PpBarvert),
    /// The `PP_CHUNK` part.
    PpChunk(PpChunk),
    /// The `PP_CHUNKVERT` part.
    PpChunkvert(PpChunkvert),
    /// The `PP_FILL` part.
    PpFill(PpFill),
    /// The `PP_FILLVERT` part.
    PpFillvert(PpFillvert),
    /// The `PP_MOVEOVERLAY` part.
    PpMoveoverlay(PpMoveoverlay),
    /// The `PP_MOVEOVERLAYVERT` part.
    PpMoveoverlayvert(PpMoveoverlayvert),
    /// The `PP_PULSEOVERLAY` part.
    PpPulseoverlay(PpPulseoverlay),
    /// The `PP_PULSEOVERLAYVERT` part.
    PpPulseoverlayvert(PpPulseoverlayvert),
    /// The `PP_TRANSPARENTBAR` part.
    PpTransparentbar(PpTransparentbar),
    /// The `PP_TRANSPARENTBARVERT` part.
    PpTransparentbarvert(PpTransparentbarvert),
    /// The `RP_BACKGROUND` part.
    RpBackground(RpBackground),
    /// The `RP_BAND` part.
    RpBand(RpBand),
    /// The `RP_CHEVRON` part.
    RpChevron(RpChevron),
    /// The `RP_CHEVRONVERT` part.
    RpChevronvert(RpChevronvert),
    /// The `RP_GRIPPER` part.
    RpGripper(RpGripper),
    /// The `RP_GRIPPERVERT` part.
    RpGrippervert(RpGrippervert),
    /// The `RP_SPLITTER` part.
    RpSplitter(RpSplitter),
    /// The `RP_SPLITTERVERT` part.
    RpSplittervert(RpSplittervert),
    /// The `SBP_ARROWBTN` part.
    SbpArrowbtn(SbpArrowbtn),
    /// The `SBP_GRIPPERHORZ` part.
    SbpGripperhorz(SbpGripperhorz),
    /// The `SBP_GRIPPERVERT` part.
    SbpGrippervert(SbpGrippervert),
    /// The `SBP_LOWERTRACKHORZ` part.
    SbpLowertrackhorz(SbpLowertrackhorz),
    /// The `SBP_LOWERTRACKVERT` part.
    SbpLowertrackvert(SbpLowertrackvert),
    /// The `SBP_SIZEBOX` part.
    SbpSizebox(SbpSizebox),
    /// The `SBP_THUMBBTNHORZ` part.
    SbpThumbbtnhorz(SbpThumbbtnhorz),
    /// The `SBP_THUMBBTNVERT` part.
    SbpThumbbtnvert(SbpThumbbtnvert),
    /// The `SBP_UPPERTRACKHORZ` part.
    SbpUppertrackhorz(SbpUppertrackhorz),
    /// The `SBP_UPPERTRACKVERT` part.
    SbpUppertrackvert(SbpUppertrackvert),
    /// The `SPNP_DOWN` part.
    SpnpDown(SpnpDown),
    /// The `SPNP_DOWNHORZ` part.
    SpnpDownhorz(SpnpDownhorz),
    /// The `SPNP_UP` part.
    SpnpUp(SpnpUp),
    /// The `SPNP_UPHORZ` part.
    SpnpUphorz(SpnpUphorz),
    /// The `SPP_LOGOFF` part.
    SppLogoff(SppLogoff),
    /// The `SPP_LOGOFFBUTTONS` part.
    SppLogoffbuttons(SppLogoffbuttons),
    /// The `SPP_MOREPROGRAMS` part.
    SppMoreprograms(SppMoreprograms),
    /// The `SPP_MOREPROGRAMSARROW` part.
    SppMoreprogramsarrow(SppMoreprogramsarrow),
    /// The `SPP_PLACESLIST` part.
    SppPlaceslist(SppPlaceslist),
    /// The `SPP_PLACESLISTSEPARATOR` part.
    SppPlaceslistseparator(SppPlaceslistseparator),
    /// The `SPP_PREVIEW` part.
    SppPreview(SppPreview),
    /// The `SPP_PROGLIST` part.
    SppProglist(SppProglist),
    /// The `SPP_PROGLISTSEPARATOR` part.
    SppProglistseparator(SppProglistseparator),
    /// The `SPP_USERPANE` part.
    SppUserpane(SppUserpane),
    /// The `SPP_USERPICTURE` part.
    SppUserpicture(SppUserpicture),
    /// The `SP_GRIPPER` part.
    SpGripper(SpGripper),
    /// The `SP_GRIPPERPANE` part.
    SpGripperpane(SpGripperpane),
    /// The `SP_PANE` part.
    SpPane(SpPane),
    /// The `TABP_AEROWIZARDBODY` part.
    TabpAerowizardbody(TabpAerowizardbody),
    /// The `TABP_BODY` part.
    TabpBody(TabpBody),
    /// The `TABP_PANE` part.
    TabpPane(TabpPane),
    /// The `TABP_TABITEM` part.
    TabpTabitem(TabpTabitem),
    /// The `TABP_TABITEMBOTHEDGE` part.
    TabpTabitembothedge(TabpTabitembothedge),
    /// The `TABP_TABITEMLEFTEDGE` part.
    TabpTabitemleftedge(TabpTabitemleftedge),
    /// The `TABP_TABITEMRIGHTEDGE` part.
    TabpTabitemrightedge(TabpTabitemrightedge),
    /// The `TABP_TOPTABITEM` part.
    TabpToptabitem(TabpToptabitem),
    /// The `TABP_TOPTABITEMBOTHEDGE` part.
    TabpToptabitembothedge(TabpToptabitembothedge),
    /// The `TABP_TOPTABITEMLEFTEDGE` part.
    TabpToptabitemleftedge(TabpToptabitemleftedge),
    /// The `TABP_TOPTABITEMRIGHTEDGE` part.
    TabpToptabitemrightedge(TabpToptabitemrightedge),
    /// The `TDP_GROUPCOUNT` part.
    TdpGroupcount(TdpGroupcount),
    /// The `TDP_FLASHBUTTON` part.
    TdpFlashbutton(TdpFlashbutton),
    /// The `TDP_FLASHBUTTONGROUPMENU` part.
    TdpFlashbuttongroupmenu(TdpFlashbuttongroupmenu),
    /// The `TBP_BACKGROUNDBOTTOM` part.
    TbpBackgroundbottom(TbpBackgroundbottom),
    /// The `TBP_BACKGROUNDLEFT` part.
    TbpBackgroundleft(TbpBackgroundleft),
    /// The `TBP_BACKGROUNDRIGHT` part.
    TbpBackgroundright(TbpBackgroundright),
    /// The `TBP_BACKGROUNDTOP` part.
    TbpBackgroundtop(TbpBackgroundtop),
    /// The `TBP_SIZINGBARBOTTOM` part.
    TbpSizingbarbottom(TbpSizingbarbottom),
    /// The `TBP_SIZINGBARRIGHT` part.
    TbpSizingbarright(TbpSizingbarright),
    /// The `TBP_SIZINGBARTOP` part.
    TbpSizingbartop(TbpSizingbartop),
    /// The `TDLG_BUTTONSECTION` part.
    TdlgButtonsection(TdlgButtonsection),
    /// The `TDLG_BUTTONWRAPPER` part.
    TdlgButtonwrapper(TdlgButtonwrapper),
    /// The `TDLG_COMMANDLINKPANE` part.
    TdlgCommandlinkpane(TdlgCommandlinkpane),
    /// The `TDLG_CONTENTICON` part.
    TdlgContenticon(TdlgContenticon),
    /// The `TDLG_CONTENTPANE` part.
    TdlgContentpane(TdlgContentpane),
    /// The `TDLG_CONTROLPANE` part.
    TdlgControlpane(TdlgControlpane),
    /// The `TDLG_EXPANDEDCONTENT` part.
    TdlgExpandedcontent(TdlgExpandedcontent),
    /// The `TDLG_EXPANDEDFOOTERAREA` part.
    TdlgExpandedfooterarea(TdlgExpandedfooterarea),
    /// The `TDLG_EXPANDOBUTTON` part.
    TdlgExpandobutton(TdlgExpandobutton),
    /// The `TDLG_EXPANDOTEXT` part.
    TdlgExpandotext(TdlgExpandotext),
    /// The `TDLG_FOOTNOTEAREA` part.
    TdlgFootnotearea(TdlgFootnotearea),
    /// The `TDLG_FOOTNOTEPANE` part.
    TdlgFootnotepane(TdlgFootnotepane),
    /// The `TDLG_FOOTNOTESEPARATOR` part.
    TdlgFootnoteseparator(TdlgFootnoteseparator),
    /// The `TDLG_IMAGEALIGNMENT` part.
    TdlgImagealignment(TdlgImagealignment),
    /// The `TDLG_MAINICON` part.
    TdlgMainicon(TdlgMainicon),
    /// The `TDLG_MAININSTRUCTIONPANE` part.
    TdlgMaininstructionpane(TdlgMaininstructionpane),
    /// The `TDLG_PRIMARYPANEL` part.
    TdlgPrimarypanel(TdlgPrimarypanel),
    /// The `TDLG_PROGRESSBAR` part.
    TdlgProgressbar(TdlgProgressbar),
    /// The `TDLG_RADIOBUTTONPANE` part.
    TdlgRadiobuttonpane(TdlgRadiobuttonpane),
    /// The `TDLG_SECONDARYPANEL` part.
    TdlgSecondarypanel(TdlgSecondarypanel),
    /// The `TDLG_VERIFICATIONTEXT` part.
    TdlgVerificationtext(TdlgVerificationtext),
    /// The `TEXT_BODYTITLE` part.
    TextBodytitle(TextBodytitle),
    /// The `TEXT_BODYTEXT` part.
    TextBodytext(TextBodytext),
    /// The `TEXT_CONTROLLABEL` part.
    TextControllabel(TextControllabel),
    /// The `TEXT_EXPANDED` part.
    TextExpanded(TextExpanded),
    /// The `TEXT_HYPERLINKTEXT` part.
    TextHyperlinktext(TextHyperlinktext),
    /// The `TEXT_INSTRUCTION` part.
    TextInstruction(TextInstruction),
    /// The `TEXT_LABEL` part.
    TextLabel(TextLabel),
    /// The `TEXT_MAININSTRUCTION` part.
    TextMaininstruction(TextMaininstruction),
    /// The `TEXT_SECONDARYTEXT` part.
    TextSecondarytext(TextSecondarytext),
    /// The `TP_BUTTON` part.
    TpButton(TpButton),
    /// The `TP_DROPDOWNBUTTON` part.
    TpDropdownbutton(TpDropdownbutton),
    /// The `TP_DROPDOWNBUTTONGLYPH` part.
    TpDropdownbuttonglyph(TpDropdownbuttonglyph),
    /// The `TP_SEPARATOR` part.
    TpSeparator(TpSeparator),
    /// The `TP_SEPARATORVERT` part.
    TpSeparatorvert(TpSeparatorvert),
    /// The `TP_SPLITBUTTON` part.
    TpSplitbutton(TpSplitbutton),
    /// The `TP_SPLITBUTTONDROPDOWN` part.
    TpSplitbuttondropdown(TpSplitbuttondropdown),
    /// The `TTP_BALLOON` part.
    TtpBalloon(TtpBalloon),
    /// The `TTP_BALLOONSTEM` part.
    TtpBalloonstem(TtpBalloonstem),
    /// The `TTP_BALLOONTITLE` part.
    TtpBalloontitle(TtpBalloontitle),
    /// The `TTP_CLOSE` part.
    TtpClose(TtpClose),
    /// The `TTP_STANDARD` part.
    TtpStandard(TtpStandard),
    /// The `TTP_STANDARDTITLE` part.
    TtpStandardtitle(TtpStandardtitle),
    /// The `TTP_WRENCH` part.
    TtpWrench(TtpWrench),
    /// The `TKP_THUMB` part.
    TkpThumb(TkpThumb),
    /// The `TKP_THUMBBOTTOM` part.
    TkpThumbbottom(TkpThumbbottom),
    /// The `TKP_THUMBLEFT` part.
    TkpThumbleft(TkpThumbleft),
    /// The `TKP_THUMBRIGHT` part.
    TkpThumbright(TkpThumbright),
    /// The `TKP_THUMBTOP` part.
    TkpThumbtop(TkpThumbtop),
    /// The `TKP_THUMBVERT` part.
    TkpThumbvert(TkpThumbvert),
    /// The `TKP_TICS` part.
    TkpTics(TkpTics),
    /// The `TKP_TICSVERT` part.
    TkpTicsvert(TkpTicsvert),
    /// The `TKP_TRACK` part.
    TkpTrack(TkpTrack),
    /// The `TKP_TRACKVERT` part.
    TkpTrackvert(TkpTrackvert),
    /// The `TNP_ANIMBACKGROUND` part.
    TnpAnimbackground(TnpAnimbackground),
    /// The `TNP_BACKGROUND` part.
    TnpBackground(TnpBackground),
    /// The `TVP_BRANCH` part.
    TvpBranch(TvpBranch),
    /// The `TVP_GLYPH` part.
    TvpGlyph(TvpGlyph),
    /// The `TVP_HOTGLYPH` part.
    TvpHotglyph(TvpHotglyph),
    /// The `TVP_TREEITEM` part.
    TvpTreeitem(TvpTreeitem),
    /// The `WP_CAPTION` part.
    WpCaption(WpCaption),
    /// The `WP_CAPTIONSIZINGTEMPLATE` part.
    WpCaptionsizingtemplate(WpCaptionsizingtemplate),
    /// The `WP_CLOSEBUTTON` part.
    WpClosebutton(WpClosebutton),
    /// The `WP_DIALOG` part.
    WpDialog(WpDialog),
    /// The `WP_FRAME` part.
    WpFrame(WpFrame),
    /// The `WP_FRAMEBOTTOM` part.
    WpFramebottom(WpFramebottom),
    /// The `WP_FRAMEBOTTOMSIZINGTEMPLATE` part.
    WpFramebottomsizingtemplate(WpFramebottomsizingtemplate),
    /// The `WP_FRAMELEFT` part.
    WpFrameleft(WpFrameleft),
    /// The `WP_FRAMELEFTSIZINGTEMPLATE` part.
    WpFrameleftsizingtemplate(WpFrameleftsizingtemplate),
    /// The `WP_FRAMERIGHT` part.
    WpFrameright(WpFrameright),
    /// The `WP_FRAMERIGHTSIZINGTEMPLATE` part.
    WpFramerightsizingtemplate(WpFramerightsizingtemplate),
    /// The `WP_HELPBUTTON` part.
    WpHelpbutton(WpHelpbutton),
    /// The `WP_HORZSCROLL` part.
    WpHorzscroll(WpHorzscroll),
    /// The `WP_HORZTHUMB` part.
    WpHorzthumb(WpHorzthumb),
    /// The `WP_MAXBUTTON` part.
    WpMaxbutton(WpMaxbutton),
    /// The `WP_MAXCAPTION` part.
    WpMaxcaption(WpMaxcaption),
    /// The `WP_MDICLOSEBUTTON` part.
    WpMdiclosebutton(WpMdiclosebutton),
    /// The `WP_MDIHELPBUTTON` part.
    WpMdihelpbutton(WpMdihelpbutton),
    /// The `WP_MDIMINBUTTON` part.
    WpMdiminbutton(WpMdiminbutton),
    /// The `WP_MDIRESTOREBUTTON` part.
    WpMdirestorebutton(WpMdirestorebutton),
    /// The `WP_MDISYSBUTTON` part.
    WpMdisysbutton(WpMdisysbutton),
    /// The `WP_MINBUTTON` part.
    WpMinbutton(WpMinbutton),
    /// The `WP_MINCAPTION` part.
    WpMincaption(WpMincaption),
    /// The `WP_RESTOREBUTTON` part.
    WpRestorebutton(WpRestorebutton),
    /// The `WP_SMALLCAPTION` part.
    WpSmallcaption(WpSmallcaption),
    /// The `WP_SMALLCAPTIONSIZINGTEMPLATE` part.
    WpSmallcaptionsizingtemplate(WpSmallcaptionsizingtemplate),
    /// The `WP_SMALLCLOSEBUTTON` part.
    WpSmallclosebutton(WpSmallclosebutton),
    /// The `WP_SMALLFRAMEBOTTOM` part.
    WpSmallframebottom(WpSmallframebottom),
    /// The `WP_SMALLFRAMEBOTTOMSIZINGTEMPLATE` part.
    WpSmallframebottomsizingtemplate(WpSmallframebottomsizingtemplate),
    /// The `WP_SMALLFRAMELEFT` part.
    WpSmallframeleft(WpSmallframeleft),
    /// The `WP_SMALLFRAMELEFTSIZINGTEMPLATE` part.
    WpSmallframeleftsizingtemplate(WpSmallframeleftsizingtemplate),
    /// The `WP_SMALLFRAMERIGHT` part.
    WpSmallframeright(WpSmallframeright),
    /// The `WP_SMALLFRAMERIGHTSIZINGTEMPLATE` part.
    WpSmallframerightsizingtemplate(WpSmallframerightsizingtemplate),
    /// The `WP_SMALLMAXCAPTION` part.
    WpSmallmaxcaption(WpSmallmaxcaption),
    /// The `WP_SMALLMINCAPTION` part.
    WpSmallmincaption(WpSmallmincaption),
    /// The `WP_SYSBUTTON` part.
    WpSysbutton(WpSysbutton),
    /// The `WP_VERTSCROLL` part.
    WpVertscroll(WpVertscroll),
    /// The `WP_VERTTHUMB` part.
    WpVertthumb(WpVertthumb),
}

/// The state of the `BP_CHECKBOX` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum BpCheckbox {
    /// The default state.
    None,
    /// The `CBS_CHECKEDDISABLED` state.
    CbsCheckeddisabled,
}

impl Default for BpCheckbox {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for BpCheckbox {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for BpCheckbox {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for BpCheckbox {}

impl PartialOrd<i32> for BpCheckbox {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for BpCheckbox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for BpCheckbox {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for BpCheckbox {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl BpCheckbox {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbsCheckeddisabled => windows_sys::Win32::UI::Controls::CBS_CHECKEDDISABLED,
        }
    }
}

/// The state of the `BP_COMMANDLINK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum BpCommandlink {
    /// The default state.
    None,
    /// The `CMDLS_DEFAULTED` state.
    CmdlsDefaulted,
}

impl Default for BpCommandlink {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for BpCommandlink {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for BpCommandlink {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for BpCommandlink {}

impl PartialOrd<i32> for BpCommandlink {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for BpCommandlink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for BpCommandlink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for BpCommandlink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl BpCommandlink {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CmdlsDefaulted => windows_sys::Win32::UI::Controls::CMDLS_DEFAULTED,
        }
    }
}

/// The state of the `BP_COMMANDLINKGLYPH` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum BpCommandlinkglyph {
    /// The default state.
    None,
    /// The `CMDLGS_DEFAULTED` state.
    CmdlgsDefaulted,
}

impl Default for BpCommandlinkglyph {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for BpCommandlinkglyph {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for BpCommandlinkglyph {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for BpCommandlinkglyph {}

impl PartialOrd<i32> for BpCommandlinkglyph {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for BpCommandlinkglyph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for BpCommandlinkglyph {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for BpCommandlinkglyph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl BpCommandlinkglyph {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CmdlgsDefaulted => windows_sys::Win32::UI::Controls::CMDLGS_DEFAULTED,
        }
    }
}

/// The state of the `BP_GROUPBOX` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum BpGroupbox {
    /// The default state.
    None,
    /// The `GBS_DISABLED` state.
    GbsDisabled,
}

impl Default for BpGroupbox {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for BpGroupbox {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for BpGroupbox {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for BpGroupbox {}

impl PartialOrd<i32> for BpGroupbox {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for BpGroupbox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for BpGroupbox {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for BpGroupbox {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl BpGroupbox {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::GbsDisabled => windows_sys::Win32::UI::Controls::GBS_DISABLED,
        }
    }
}

/// The state of the `BP_PUSHBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum BpPushbutton {
    /// The default state.
    None,
    /// The `PBS_DEFAULTED` state.
    PbsDefaulted,
}

impl Default for BpPushbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for BpPushbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for BpPushbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for BpPushbutton {}

impl PartialOrd<i32> for BpPushbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for BpPushbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for BpPushbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for BpPushbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl BpPushbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::PbsDefaulted => windows_sys::Win32::UI::Controls::PBS_DEFAULTED,
        }
    }
}

/// The state of the `BP_RADIOBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum BpRadiobutton {
    /// The default state.
    None,
    /// The `RBS_CHECKEDDISABLED` state.
    RbsCheckeddisabled,
}

impl Default for BpRadiobutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for BpRadiobutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for BpRadiobutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for BpRadiobutton {}

impl PartialOrd<i32> for BpRadiobutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for BpRadiobutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for BpRadiobutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for BpRadiobutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl BpRadiobutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::RbsCheckeddisabled => windows_sys::Win32::UI::Controls::RBS_CHECKEDDISABLED,
        }
    }
}

/// The state of the `BP_USERBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum BpUserbutton {
    /// The default state.
    None,
}

impl Default for BpUserbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for BpUserbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for BpUserbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for BpUserbutton {}

impl PartialOrd<i32> for BpUserbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for BpUserbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for BpUserbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for BpUserbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl BpUserbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CLP_TIME` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum ClpTime {
    /// The default state.
    None,
    /// The `CLS_NORMAL` state.
    ClsNormal,
}

impl Default for ClpTime {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for ClpTime {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for ClpTime {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for ClpTime {}

impl PartialOrd<i32> for ClpTime {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for ClpTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for ClpTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for ClpTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl ClpTime {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ClsNormal => windows_sys::Win32::UI::Controls::CLS_NORMAL,
        }
    }
}

/// The state of the `CP_BACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpBackground {
    /// The default state.
    None,
}

impl Default for CpBackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpBackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpBackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpBackground {}

impl PartialOrd<i32> for CpBackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpBackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpBackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpBackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpBackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CP_BORDER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpBorder {
    /// The default state.
    None,
    /// The `CBB_DISABLED` state.
    CbbDisabled,
}

impl Default for CpBorder {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpBorder {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpBorder {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpBorder {}

impl PartialOrd<i32> for CpBorder {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpBorder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpBorder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpBorder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpBorder {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbbDisabled => windows_sys::Win32::UI::Controls::CBB_DISABLED,
        }
    }
}

/// The state of the `CP_CUEBANNER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpCuebanner {
    /// The default state.
    None,
    /// The `CBCB_DISABLED` state.
    CbcbDisabled,
}

impl Default for CpCuebanner {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpCuebanner {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpCuebanner {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpCuebanner {}

impl PartialOrd<i32> for CpCuebanner {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpCuebanner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpCuebanner {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpCuebanner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpCuebanner {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbcbDisabled => windows_sys::Win32::UI::Controls::CBCB_DISABLED,
        }
    }
}

/// The state of the `CP_DROPDOWNBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpDropdownbutton {
    /// The default state.
    None,
    /// The `CBXS_DISABLED` state.
    CbxsDisabled,
}

impl Default for CpDropdownbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpDropdownbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpDropdownbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpDropdownbutton {}

impl PartialOrd<i32> for CpDropdownbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpDropdownbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpDropdownbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpDropdownbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpDropdownbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbxsDisabled => windows_sys::Win32::UI::Controls::CBXS_DISABLED,
        }
    }
}

/// The state of the `CP_DROPDOWNBUTTONLEFT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpDropdownbuttonleft {
    /// The default state.
    None,
    /// The `CBXSL_DISABLED` state.
    CbxslDisabled,
}

impl Default for CpDropdownbuttonleft {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpDropdownbuttonleft {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpDropdownbuttonleft {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpDropdownbuttonleft {}

impl PartialOrd<i32> for CpDropdownbuttonleft {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpDropdownbuttonleft {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpDropdownbuttonleft {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpDropdownbuttonleft {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpDropdownbuttonleft {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbxslDisabled => windows_sys::Win32::UI::Controls::CBXSL_DISABLED,
        }
    }
}

/// The state of the `CP_DROPDOWNBUTTONRIGHT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpDropdownbuttonright {
    /// The default state.
    None,
    /// The `CBXSR_DISABLED` state.
    CbxsrDisabled,
}

impl Default for CpDropdownbuttonright {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpDropdownbuttonright {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpDropdownbuttonright {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpDropdownbuttonright {}

impl PartialOrd<i32> for CpDropdownbuttonright {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpDropdownbuttonright {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpDropdownbuttonright {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpDropdownbuttonright {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpDropdownbuttonright {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbxsrDisabled => windows_sys::Win32::UI::Controls::CBXSR_DISABLED,
        }
    }
}

/// The state of the `CP_TRANSPARENTBACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpTransparentbackground {
    /// The default state.
    None,
    /// The `CBTBS_DISABLED` state.
    CbtbsDisabled,
}

impl Default for CpTransparentbackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpTransparentbackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpTransparentbackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpTransparentbackground {}

impl PartialOrd<i32> for CpTransparentbackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpTransparentbackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpTransparentbackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpTransparentbackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpTransparentbackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbtbsDisabled => windows_sys::Win32::UI::Controls::CBTBS_DISABLED,
        }
    }
}

/// The state of the `CP_READONLY` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpReadonly {
    /// The default state.
    None,
    /// The `CBRO_DISABLED` state.
    CbroDisabled,
}

impl Default for CpReadonly {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpReadonly {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpReadonly {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpReadonly {}

impl PartialOrd<i32> for CpReadonly {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpReadonly {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpReadonly {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpReadonly {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpReadonly {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbroDisabled => windows_sys::Win32::UI::Controls::CBRO_DISABLED,
        }
    }
}

/// The state of the `CSST_TAB` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CsstTab {
    /// The default state.
    None,
    /// The `CSTB_HOT` state.
    CstbHot,
}

impl Default for CsstTab {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CsstTab {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CsstTab {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CsstTab {}

impl PartialOrd<i32> for CsstTab {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CsstTab {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CsstTab {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CsstTab {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CsstTab {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CstbHot => windows_sys::Win32::UI::Controls::CSTB_HOT,
        }
    }
}

/// The state of the `CPANEL_BANNERAREA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelBannerarea {
    /// The default state.
    None,
}

impl Default for CpanelBannerarea {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelBannerarea {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelBannerarea {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelBannerarea {}

impl PartialOrd<i32> for CpanelBannerarea {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelBannerarea {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelBannerarea {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelBannerarea {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelBannerarea {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_BODYTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelBodytext {
    /// The default state.
    None,
}

impl Default for CpanelBodytext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelBodytext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelBodytext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelBodytext {}

impl PartialOrd<i32> for CpanelBodytext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelBodytext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelBodytext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelBodytext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelBodytext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_BODYTITLE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelBodytitle {
    /// The default state.
    None,
}

impl Default for CpanelBodytitle {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelBodytitle {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelBodytitle {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelBodytitle {}

impl PartialOrd<i32> for CpanelBodytitle {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelBodytitle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelBodytitle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelBodytitle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelBodytitle {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_BUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelButton {
    /// The default state.
    None,
}

impl Default for CpanelButton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelButton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelButton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelButton {}

impl PartialOrd<i32> for CpanelButton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelButton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelButton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelButton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelButton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_CONTENTLINK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelContentlink {
    /// The default state.
    None,
    /// The `CPCL_DISABLED` state.
    CpclDisabled,
}

impl Default for CpanelContentlink {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelContentlink {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelContentlink {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelContentlink {}

impl PartialOrd<i32> for CpanelContentlink {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelContentlink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelContentlink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelContentlink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelContentlink {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CpclDisabled => windows_sys::Win32::UI::Controls::CPCL_DISABLED,
        }
    }
}

/// The state of the `CPANEL_CONTENTPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelContentpane {
    /// The default state.
    None,
}

impl Default for CpanelContentpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelContentpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelContentpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelContentpane {}

impl PartialOrd<i32> for CpanelContentpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelContentpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelContentpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelContentpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelContentpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_CONTENTPANELABEL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelContentpanelabel {
    /// The default state.
    None,
}

impl Default for CpanelContentpanelabel {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelContentpanelabel {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelContentpanelabel {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelContentpanelabel {}

impl PartialOrd<i32> for CpanelContentpanelabel {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelContentpanelabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelContentpanelabel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelContentpanelabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelContentpanelabel {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_CONTENTPANELINE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelContentpaneline {
    /// The default state.
    None,
}

impl Default for CpanelContentpaneline {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelContentpaneline {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelContentpaneline {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelContentpaneline {}

impl PartialOrd<i32> for CpanelContentpaneline {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelContentpaneline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelContentpaneline {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelContentpaneline {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelContentpaneline {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_GROUPTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelGrouptext {
    /// The default state.
    None,
}

impl Default for CpanelGrouptext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelGrouptext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelGrouptext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelGrouptext {}

impl PartialOrd<i32> for CpanelGrouptext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelGrouptext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelGrouptext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelGrouptext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelGrouptext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_HELPLINK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelHelplink {
    /// The default state.
    None,
    /// The `CPHL_DISABLED` state.
    CphlDisabled,
}

impl Default for CpanelHelplink {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelHelplink {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelHelplink {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelHelplink {}

impl PartialOrd<i32> for CpanelHelplink {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelHelplink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelHelplink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelHelplink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelHelplink {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CphlDisabled => windows_sys::Win32::UI::Controls::CPHL_DISABLED,
        }
    }
}

/// The state of the `CPANEL_LARGECOMMANDAREA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelLargecommandarea {
    /// The default state.
    None,
}

impl Default for CpanelLargecommandarea {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelLargecommandarea {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelLargecommandarea {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelLargecommandarea {}

impl PartialOrd<i32> for CpanelLargecommandarea {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelLargecommandarea {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelLargecommandarea {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelLargecommandarea {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelLargecommandarea {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_MESSAGETEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelMessagetext {
    /// The default state.
    None,
}

impl Default for CpanelMessagetext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelMessagetext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelMessagetext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelMessagetext {}

impl PartialOrd<i32> for CpanelMessagetext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelMessagetext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelMessagetext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelMessagetext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelMessagetext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_NAVIGATIONPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelNavigationpane {
    /// The default state.
    None,
}

impl Default for CpanelNavigationpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelNavigationpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelNavigationpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelNavigationpane {}

impl PartialOrd<i32> for CpanelNavigationpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelNavigationpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelNavigationpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelNavigationpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelNavigationpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_NAVIGATIONPANELABEL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelNavigationpanelabel {
    /// The default state.
    None,
}

impl Default for CpanelNavigationpanelabel {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelNavigationpanelabel {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelNavigationpanelabel {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelNavigationpanelabel {}

impl PartialOrd<i32> for CpanelNavigationpanelabel {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelNavigationpanelabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelNavigationpanelabel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelNavigationpanelabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelNavigationpanelabel {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_NAVIGATIONPANELINE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelNavigationpaneline {
    /// The default state.
    None,
}

impl Default for CpanelNavigationpaneline {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelNavigationpaneline {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelNavigationpaneline {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelNavigationpaneline {}

impl PartialOrd<i32> for CpanelNavigationpaneline {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelNavigationpaneline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelNavigationpaneline {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelNavigationpaneline {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelNavigationpaneline {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_SECTIONTITLELINK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelSectiontitlelink {
    /// The default state.
    None,
    /// The `CPSTL_HOT` state.
    CpstlHot,
}

impl Default for CpanelSectiontitlelink {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelSectiontitlelink {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelSectiontitlelink {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelSectiontitlelink {}

impl PartialOrd<i32> for CpanelSectiontitlelink {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelSectiontitlelink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelSectiontitlelink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelSectiontitlelink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelSectiontitlelink {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CpstlHot => windows_sys::Win32::UI::Controls::CPSTL_HOT,
        }
    }
}

/// The state of the `CPANEL_SMALLCOMMANDAREA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelSmallcommandarea {
    /// The default state.
    None,
}

impl Default for CpanelSmallcommandarea {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelSmallcommandarea {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelSmallcommandarea {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelSmallcommandarea {}

impl PartialOrd<i32> for CpanelSmallcommandarea {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelSmallcommandarea {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelSmallcommandarea {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelSmallcommandarea {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelSmallcommandarea {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `CPANEL_TASKLINK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelTasklink {
    /// The default state.
    None,
    /// The `CPTL_DISABLED` state.
    CptlDisabled,
}

impl Default for CpanelTasklink {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelTasklink {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelTasklink {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelTasklink {}

impl PartialOrd<i32> for CpanelTasklink {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelTasklink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelTasklink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelTasklink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelTasklink {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CptlDisabled => windows_sys::Win32::UI::Controls::CPTL_DISABLED,
        }
    }
}

/// The state of the `CPANEL_TITLE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CpanelTitle {
    /// The default state.
    None,
}

impl Default for CpanelTitle {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for CpanelTitle {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for CpanelTitle {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for CpanelTitle {}

impl PartialOrd<i32> for CpanelTitle {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for CpanelTitle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for CpanelTitle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for CpanelTitle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl CpanelTitle {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `DP_DATEBORDER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DpDateborder {
    /// The default state.
    None,
    /// The `DPDB_DISABLED` state.
    DpdbDisabled,
}

impl Default for DpDateborder {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DpDateborder {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DpDateborder {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DpDateborder {}

impl PartialOrd<i32> for DpDateborder {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DpDateborder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DpDateborder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DpDateborder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DpDateborder {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DpdbDisabled => windows_sys::Win32::UI::Controls::DPDB_DISABLED,
        }
    }
}

/// The state of the `DP_DATETEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DpDatetext {
    /// The default state.
    None,
    /// The `DPDT_DISABLED` state.
    DpdtDisabled,
}

impl Default for DpDatetext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DpDatetext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DpDatetext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DpDatetext {}

impl PartialOrd<i32> for DpDatetext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DpDatetext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DpDatetext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DpDatetext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DpDatetext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DpdtDisabled => windows_sys::Win32::UI::Controls::DPDT_DISABLED,
        }
    }
}

/// The state of the `DP_SHOWCALENDARBUTTONRIGHT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DpShowcalendarbuttonright {
    /// The default state.
    None,
    /// The `DPSCBR_DISABLED` state.
    DpscbrDisabled,
}

impl Default for DpShowcalendarbuttonright {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DpShowcalendarbuttonright {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DpShowcalendarbuttonright {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DpShowcalendarbuttonright {}

impl PartialOrd<i32> for DpShowcalendarbuttonright {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DpShowcalendarbuttonright {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DpShowcalendarbuttonright {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DpShowcalendarbuttonright {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DpShowcalendarbuttonright {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DpscbrDisabled => windows_sys::Win32::UI::Controls::DPSCBR_DISABLED,
        }
    }
}

/// The state of the `DD_COPY` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DdCopy {
    /// The default state.
    None,
    /// The `DDCOPY_HIGHLIGHT` state.
    DdcopyHighlight,
}

impl Default for DdCopy {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DdCopy {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DdCopy {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DdCopy {}

impl PartialOrd<i32> for DdCopy {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DdCopy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DdCopy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DdCopy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DdCopy {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DdcopyHighlight => windows_sys::Win32::UI::Controls::DDCOPY_HIGHLIGHT,
        }
    }
}

/// The state of the `DD_CREATELINK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DdCreatelink {
    /// The default state.
    None,
    /// The `DDCREATELINK_HIGHLIGHT` state.
    DdcreatelinkHighlight,
}

impl Default for DdCreatelink {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DdCreatelink {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DdCreatelink {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DdCreatelink {}

impl PartialOrd<i32> for DdCreatelink {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DdCreatelink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DdCreatelink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DdCreatelink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DdCreatelink {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DdcreatelinkHighlight => windows_sys::Win32::UI::Controls::DDCREATELINK_HIGHLIGHT,
        }
    }
}

/// The state of the `DD_IMAGEBG` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DdImagebg {
    /// The default state.
    None,
}

impl Default for DdImagebg {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DdImagebg {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DdImagebg {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DdImagebg {}

impl PartialOrd<i32> for DdImagebg {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DdImagebg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DdImagebg {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DdImagebg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DdImagebg {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `DD_MOVE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DdMove {
    /// The default state.
    None,
    /// The `DDMOVE_HIGHLIGHT` state.
    DdmoveHighlight,
}

impl Default for DdMove {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DdMove {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DdMove {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DdMove {}

impl PartialOrd<i32> for DdMove {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DdMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DdMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DdMove {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DdMove {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DdmoveHighlight => windows_sys::Win32::UI::Controls::DDMOVE_HIGHLIGHT,
        }
    }
}

/// The state of the `DD_NONE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DdNone {
    /// The default state.
    None,
    /// The `DDNONE_HIGHLIGHT` state.
    DdnoneHighlight,
}

impl Default for DdNone {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DdNone {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DdNone {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DdNone {}

impl PartialOrd<i32> for DdNone {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DdNone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DdNone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DdNone {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DdNone {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DdnoneHighlight => windows_sys::Win32::UI::Controls::DDNONE_HIGHLIGHT,
        }
    }
}

/// The state of the `DD_TEXTBG` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DdTextbg {
    /// The default state.
    None,
}

impl Default for DdTextbg {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DdTextbg {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DdTextbg {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DdTextbg {}

impl PartialOrd<i32> for DdTextbg {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DdTextbg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DdTextbg {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DdTextbg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DdTextbg {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `DD_UPDATEMETADATA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DdUpdatemetadata {
    /// The default state.
    None,
    /// The `DDUPDATEMETADATA_HIGHLIGHT` state.
    DdupdatemetadataHighlight,
}

impl Default for DdUpdatemetadata {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DdUpdatemetadata {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DdUpdatemetadata {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DdUpdatemetadata {}

impl PartialOrd<i32> for DdUpdatemetadata {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DdUpdatemetadata {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DdUpdatemetadata {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DdUpdatemetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DdUpdatemetadata {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DdupdatemetadataHighlight => windows_sys::Win32::UI::Controls::DDUPDATEMETADATA_HIGHLIGHT,
        }
    }
}

/// The state of the `DD_WARNING` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DdWarning {
    /// The default state.
    None,
    /// The `DDWARNING_HIGHLIGHT` state.
    DdwarningHighlight,
}

impl Default for DdWarning {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for DdWarning {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for DdWarning {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for DdWarning {}

impl PartialOrd<i32> for DdWarning {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for DdWarning {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for DdWarning {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for DdWarning {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl DdWarning {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DdwarningHighlight => windows_sys::Win32::UI::Controls::DDWARNING_HIGHLIGHT,
        }
    }
}

/// The state of the `EP_BACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpBackground {
    /// The default state.
    None,
    /// The `EBS_ASSIST` state.
    EbsAssist,
}

impl Default for EpBackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpBackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpBackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpBackground {}

impl PartialOrd<i32> for EpBackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpBackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpBackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpBackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpBackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbsAssist => windows_sys::Win32::UI::Controls::EBS_ASSIST,
        }
    }
}

/// The state of the `EP_BACKGROUNDWITHBORDER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpBackgroundwithborder {
    /// The default state.
    None,
    /// The `EBWBS_DISABLED` state.
    EbwbsDisabled,
}

impl Default for EpBackgroundwithborder {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpBackgroundwithborder {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpBackgroundwithborder {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpBackgroundwithborder {}

impl PartialOrd<i32> for EpBackgroundwithborder {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpBackgroundwithborder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpBackgroundwithborder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpBackgroundwithborder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpBackgroundwithborder {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbwbsDisabled => windows_sys::Win32::UI::Controls::EBWBS_DISABLED,
        }
    }
}

/// The state of the `EP_CARET` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpCaret {
    /// The default state.
    None,
}

impl Default for EpCaret {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpCaret {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpCaret {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpCaret {}

impl PartialOrd<i32> for EpCaret {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpCaret {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpCaret {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpCaret {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpCaret {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `EP_EDITBORDER_HSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpEditborderHscroll {
    /// The default state.
    None,
    /// The `EPSH_DISABLED` state.
    EpshDisabled,
}

impl Default for EpEditborderHscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpEditborderHscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpEditborderHscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpEditborderHscroll {}

impl PartialOrd<i32> for EpEditborderHscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpEditborderHscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpEditborderHscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpEditborderHscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpEditborderHscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EpshDisabled => windows_sys::Win32::UI::Controls::EPSH_DISABLED,
        }
    }
}

/// The state of the `EP_EDITBORDER_HVSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpEditborderHvscroll {
    /// The default state.
    None,
    /// The `EPSHV_DISABLED` state.
    EpshvDisabled,
}

impl Default for EpEditborderHvscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpEditborderHvscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpEditborderHvscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpEditborderHvscroll {}

impl PartialOrd<i32> for EpEditborderHvscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpEditborderHvscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpEditborderHvscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpEditborderHvscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpEditborderHvscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EpshvDisabled => windows_sys::Win32::UI::Controls::EPSHV_DISABLED,
        }
    }
}

/// The state of the `EP_EDITBORDER_NOSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpEditborderNoscroll {
    /// The default state.
    None,
    /// The `EPSN_DISABLED` state.
    EpsnDisabled,
}

impl Default for EpEditborderNoscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpEditborderNoscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpEditborderNoscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpEditborderNoscroll {}

impl PartialOrd<i32> for EpEditborderNoscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpEditborderNoscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpEditborderNoscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpEditborderNoscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpEditborderNoscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EpsnDisabled => windows_sys::Win32::UI::Controls::EPSN_DISABLED,
        }
    }
}

/// The state of the `EP_EDITBORDER_VSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpEditborderVscroll {
    /// The default state.
    None,
    /// The `EPSV_DISABLED` state.
    EpsvDisabled,
}

impl Default for EpEditborderVscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpEditborderVscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpEditborderVscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpEditborderVscroll {}

impl PartialOrd<i32> for EpEditborderVscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpEditborderVscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpEditborderVscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpEditborderVscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpEditborderVscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EpsvDisabled => windows_sys::Win32::UI::Controls::EPSV_DISABLED,
        }
    }
}

/// The state of the `EP_EDITTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpEdittext {
    /// The default state.
    None,
    /// The `ETS_ASSIST` state.
    EtsAssist,
}

impl Default for EpEdittext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpEdittext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpEdittext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpEdittext {}

impl PartialOrd<i32> for EpEdittext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpEdittext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpEdittext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpEdittext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpEdittext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EtsAssist => windows_sys::Win32::UI::Controls::ETS_ASSIST,
        }
    }
}

/// The state of the `EP_PASSWORD` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EpPassword {
    /// The default state.
    None,
}

impl Default for EpPassword {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EpPassword {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EpPassword {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EpPassword {}

impl PartialOrd<i32> for EpPassword {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EpPassword {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EpPassword {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EpPassword {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EpPassword {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `EBP_HEADERBACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpHeaderbackground {
    /// The default state.
    None,
}

impl Default for EbpHeaderbackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpHeaderbackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpHeaderbackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpHeaderbackground {}

impl PartialOrd<i32> for EbpHeaderbackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpHeaderbackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpHeaderbackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpHeaderbackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpHeaderbackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `EBP_HEADERCLOSE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpHeaderclose {
    /// The default state.
    None,
    /// The `EBHC_HOT` state.
    EbhcHot,
}

impl Default for EbpHeaderclose {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpHeaderclose {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpHeaderclose {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpHeaderclose {}

impl PartialOrd<i32> for EbpHeaderclose {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpHeaderclose {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpHeaderclose {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpHeaderclose {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpHeaderclose {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbhcHot => windows_sys::Win32::UI::Controls::EBHC_HOT,
        }
    }
}

/// The state of the `EBP_HEADERPIN` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpHeaderpin {
    /// The default state.
    None,
    /// The `EBHP_HOT` state.
    EbhpHot,
}

impl Default for EbpHeaderpin {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpHeaderpin {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpHeaderpin {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpHeaderpin {}

impl PartialOrd<i32> for EbpHeaderpin {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpHeaderpin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpHeaderpin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpHeaderpin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpHeaderpin {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbhpHot => windows_sys::Win32::UI::Controls::EBHP_HOT,
        }
    }
}

/// The state of the `EBP_IEBARMENU` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpIebarmenu {
    /// The default state.
    None,
    /// The `EBM_HOT` state.
    EbmHot,
}

impl Default for EbpIebarmenu {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpIebarmenu {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpIebarmenu {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpIebarmenu {}

impl PartialOrd<i32> for EbpIebarmenu {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpIebarmenu {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpIebarmenu {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpIebarmenu {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpIebarmenu {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbmHot => windows_sys::Win32::UI::Controls::EBM_HOT,
        }
    }
}

/// The state of the `EBP_NORMALGROUPBACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpNormalgroupbackground {
    /// The default state.
    None,
}

impl Default for EbpNormalgroupbackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpNormalgroupbackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpNormalgroupbackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpNormalgroupbackground {}

impl PartialOrd<i32> for EbpNormalgroupbackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpNormalgroupbackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpNormalgroupbackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpNormalgroupbackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpNormalgroupbackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `EBP_NORMALGROUPCOLLAPSE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpNormalgroupcollapse {
    /// The default state.
    None,
    /// The `EBNGC_HOT` state.
    EbngcHot,
}

impl Default for EbpNormalgroupcollapse {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpNormalgroupcollapse {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpNormalgroupcollapse {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpNormalgroupcollapse {}

impl PartialOrd<i32> for EbpNormalgroupcollapse {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpNormalgroupcollapse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpNormalgroupcollapse {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpNormalgroupcollapse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpNormalgroupcollapse {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbngcHot => windows_sys::Win32::UI::Controls::EBNGC_HOT,
        }
    }
}

/// The state of the `EBP_NORMALGROUPEXPAND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpNormalgroupexpand {
    /// The default state.
    None,
    /// The `EBNGE_HOT` state.
    EbngeHot,
}

impl Default for EbpNormalgroupexpand {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpNormalgroupexpand {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpNormalgroupexpand {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpNormalgroupexpand {}

impl PartialOrd<i32> for EbpNormalgroupexpand {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpNormalgroupexpand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpNormalgroupexpand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpNormalgroupexpand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpNormalgroupexpand {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbngeHot => windows_sys::Win32::UI::Controls::EBNGE_HOT,
        }
    }
}

/// The state of the `EBP_NORMALGROUPHEAD` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpNormalgrouphead {
    /// The default state.
    None,
}

impl Default for EbpNormalgrouphead {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpNormalgrouphead {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpNormalgrouphead {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpNormalgrouphead {}

impl PartialOrd<i32> for EbpNormalgrouphead {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpNormalgrouphead {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpNormalgrouphead {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpNormalgrouphead {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpNormalgrouphead {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `EBP_SPECIALGROUPBACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpSpecialgroupbackground {
    /// The default state.
    None,
}

impl Default for EbpSpecialgroupbackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpSpecialgroupbackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpSpecialgroupbackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpSpecialgroupbackground {}

impl PartialOrd<i32> for EbpSpecialgroupbackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpSpecialgroupbackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpSpecialgroupbackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpSpecialgroupbackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpSpecialgroupbackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `EBP_SPECIALGROUPCOLLAPSE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpSpecialgroupcollapse {
    /// The default state.
    None,
    /// The `EBSGC_HOT` state.
    EbsgcHot,
}

impl Default for EbpSpecialgroupcollapse {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpSpecialgroupcollapse {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpSpecialgroupcollapse {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpSpecialgroupcollapse {}

impl PartialOrd<i32> for EbpSpecialgroupcollapse {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpSpecialgroupcollapse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpSpecialgroupcollapse {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpSpecialgroupcollapse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpSpecialgroupcollapse {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbsgcHot => windows_sys::Win32::UI::Controls::EBSGC_HOT,
        }
    }
}

/// The state of the `EBP_SPECIALGROUPEXPAND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpSpecialgroupexpand {
    /// The default state.
    None,
    /// The `EBSGE_HOT` state.
    EbsgeHot,
}

impl Default for EbpSpecialgroupexpand {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpSpecialgroupexpand {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpSpecialgroupexpand {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpSpecialgroupexpand {}

impl PartialOrd<i32> for EbpSpecialgroupexpand {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpSpecialgroupexpand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpSpecialgroupexpand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpSpecialgroupexpand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpSpecialgroupexpand {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::EbsgeHot => windows_sys::Win32::UI::Controls::EBSGE_HOT,
        }
    }
}

/// The state of the `EBP_SPECIALGROUPHEAD` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum EbpSpecialgrouphead {
    /// The default state.
    None,
}

impl Default for EbpSpecialgrouphead {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for EbpSpecialgrouphead {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for EbpSpecialgrouphead {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for EbpSpecialgrouphead {}

impl PartialOrd<i32> for EbpSpecialgrouphead {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for EbpSpecialgrouphead {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for EbpSpecialgrouphead {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for EbpSpecialgrouphead {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl EbpSpecialgrouphead {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `FLYOUT_BODY` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum FlyoutBody {
    /// The default state.
    None,
    /// The `FBS_EMPHASIZED` state.
    FbsEmphasized,
}

impl Default for FlyoutBody {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for FlyoutBody {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for FlyoutBody {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for FlyoutBody {}

impl PartialOrd<i32> for FlyoutBody {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for FlyoutBody {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for FlyoutBody {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for FlyoutBody {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl FlyoutBody {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::FbsEmphasized => windows_sys::Win32::UI::Controls::FBS_EMPHASIZED,
        }
    }
}

/// The state of the `FLYOUT_DIVIDER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum FlyoutDivider {
    /// The default state.
    None,
}

impl Default for FlyoutDivider {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for FlyoutDivider {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for FlyoutDivider {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for FlyoutDivider {}

impl PartialOrd<i32> for FlyoutDivider {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for FlyoutDivider {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for FlyoutDivider {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for FlyoutDivider {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl FlyoutDivider {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `FLYOUT_HEADER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum FlyoutHeader {
    /// The default state.
    None,
}

impl Default for FlyoutHeader {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for FlyoutHeader {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for FlyoutHeader {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for FlyoutHeader {}

impl PartialOrd<i32> for FlyoutHeader {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for FlyoutHeader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for FlyoutHeader {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for FlyoutHeader {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl FlyoutHeader {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `FLYOUT_LABEL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum FlyoutLabel {
    /// The default state.
    None,
    /// The `FLS_DISABLED` state.
    FlsDisabled,
}

impl Default for FlyoutLabel {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for FlyoutLabel {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for FlyoutLabel {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for FlyoutLabel {}

impl PartialOrd<i32> for FlyoutLabel {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for FlyoutLabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for FlyoutLabel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for FlyoutLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl FlyoutLabel {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::FlsDisabled => windows_sys::Win32::UI::Controls::FLS_DISABLED,
        }
    }
}

/// The state of the `FLYOUT_LINK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum FlyoutLink {
    /// The default state.
    None,
    /// The `FLYOUTLINK_HOVER` state.
    FlyoutlinkHover,
}

impl Default for FlyoutLink {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for FlyoutLink {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for FlyoutLink {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for FlyoutLink {}

impl PartialOrd<i32> for FlyoutLink {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for FlyoutLink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for FlyoutLink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for FlyoutLink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl FlyoutLink {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::FlyoutlinkHover => windows_sys::Win32::UI::Controls::FLYOUTLINK_HOVER,
        }
    }
}

/// The state of the `FLYOUT_LINKAREA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum FlyoutLinkarea {
    /// The default state.
    None,
}

impl Default for FlyoutLinkarea {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for FlyoutLinkarea {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for FlyoutLinkarea {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for FlyoutLinkarea {}

impl PartialOrd<i32> for FlyoutLinkarea {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for FlyoutLinkarea {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for FlyoutLinkarea {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for FlyoutLinkarea {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl FlyoutLinkarea {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `FLYOUT_LINKHEADER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum FlyoutLinkheader {
    /// The default state.
    None,
    /// The `FLH_HOVER` state.
    FlhHover,
}

impl Default for FlyoutLinkheader {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for FlyoutLinkheader {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for FlyoutLinkheader {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for FlyoutLinkheader {}

impl PartialOrd<i32> for FlyoutLinkheader {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for FlyoutLinkheader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for FlyoutLinkheader {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for FlyoutLinkheader {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl FlyoutLinkheader {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::FlhHover => windows_sys::Win32::UI::Controls::FLH_HOVER,
        }
    }
}

/// The state of the `FLYOUT_WINDOW` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum FlyoutWindow {
    /// The default state.
    None,
}

impl Default for FlyoutWindow {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for FlyoutWindow {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for FlyoutWindow {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for FlyoutWindow {}

impl PartialOrd<i32> for FlyoutWindow {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for FlyoutWindow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for FlyoutWindow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for FlyoutWindow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl FlyoutWindow {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `HP_HEADERDROPDOWN` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum HpHeaderdropdown {
    /// The default state.
    None,
    /// The `HDDS_HOT` state.
    HddsHot,
}

impl Default for HpHeaderdropdown {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for HpHeaderdropdown {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for HpHeaderdropdown {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for HpHeaderdropdown {}

impl PartialOrd<i32> for HpHeaderdropdown {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for HpHeaderdropdown {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for HpHeaderdropdown {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for HpHeaderdropdown {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl HpHeaderdropdown {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HddsHot => windows_sys::Win32::UI::Controls::HDDS_HOT,
        }
    }
}

/// The state of the `HP_HEADERDROPDOWNFILTER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum HpHeaderdropdownfilter {
    /// The default state.
    None,
    /// The `HDDFS_HOT` state.
    HddfsHot,
}

impl Default for HpHeaderdropdownfilter {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for HpHeaderdropdownfilter {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for HpHeaderdropdownfilter {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for HpHeaderdropdownfilter {}

impl PartialOrd<i32> for HpHeaderdropdownfilter {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for HpHeaderdropdownfilter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for HpHeaderdropdownfilter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for HpHeaderdropdownfilter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl HpHeaderdropdownfilter {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HddfsHot => windows_sys::Win32::UI::Controls::HDDFS_HOT,
        }
    }
}

/// The state of the `HP_HEADERITEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum HpHeaderitem {
    /// The default state.
    None,
    /// The `HIS_HOT` state.
    HisHot,
}

impl Default for HpHeaderitem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for HpHeaderitem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for HpHeaderitem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for HpHeaderitem {}

impl PartialOrd<i32> for HpHeaderitem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for HpHeaderitem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for HpHeaderitem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for HpHeaderitem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl HpHeaderitem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HisHot => windows_sys::Win32::UI::Controls::HIS_HOT,
        }
    }
}

/// The state of the `HP_HEADERITEMLEFT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum HpHeaderitemleft {
    /// The default state.
    None,
    /// The `HILS_HOT` state.
    HilsHot,
}

impl Default for HpHeaderitemleft {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for HpHeaderitemleft {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for HpHeaderitemleft {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for HpHeaderitemleft {}

impl PartialOrd<i32> for HpHeaderitemleft {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for HpHeaderitemleft {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for HpHeaderitemleft {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for HpHeaderitemleft {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl HpHeaderitemleft {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HilsHot => windows_sys::Win32::UI::Controls::HILS_HOT,
        }
    }
}

/// The state of the `HP_HEADERITEMRIGHT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum HpHeaderitemright {
    /// The default state.
    None,
    /// The `HIRS_HOT` state.
    HirsHot,
}

impl Default for HpHeaderitemright {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for HpHeaderitemright {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for HpHeaderitemright {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for HpHeaderitemright {}

impl PartialOrd<i32> for HpHeaderitemright {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for HpHeaderitemright {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for HpHeaderitemright {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for HpHeaderitemright {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl HpHeaderitemright {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HirsHot => windows_sys::Win32::UI::Controls::HIRS_HOT,
        }
    }
}

/// The state of the `HP_HEADEROVERFLOW` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum HpHeaderoverflow {
    /// The default state.
    None,
    /// The `HOFS_HOT` state.
    HofsHot,
}

impl Default for HpHeaderoverflow {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for HpHeaderoverflow {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for HpHeaderoverflow {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for HpHeaderoverflow {}

impl PartialOrd<i32> for HpHeaderoverflow {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for HpHeaderoverflow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for HpHeaderoverflow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for HpHeaderoverflow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl HpHeaderoverflow {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HofsHot => windows_sys::Win32::UI::Controls::HOFS_HOT,
        }
    }
}

/// The state of the `HP_HEADERSORTARROW` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum HpHeadersortarrow {
    /// The default state.
    None,
    /// The `HSAS_SORTEDDOWN` state.
    HsasSorteddown,
}

impl Default for HpHeadersortarrow {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for HpHeadersortarrow {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for HpHeadersortarrow {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for HpHeadersortarrow {}

impl PartialOrd<i32> for HpHeadersortarrow {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for HpHeadersortarrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for HpHeadersortarrow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for HpHeadersortarrow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl HpHeadersortarrow {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HsasSorteddown => windows_sys::Win32::UI::Controls::HSAS_SORTEDDOWN,
        }
    }
}

/// The state of the `LBCP_BORDER_HSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LbcpBorderHscroll {
    /// The default state.
    None,
    /// The `LBPSH_DISABLED` state.
    LbpshDisabled,
}

impl Default for LbcpBorderHscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LbcpBorderHscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LbcpBorderHscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LbcpBorderHscroll {}

impl PartialOrd<i32> for LbcpBorderHscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LbcpBorderHscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LbcpBorderHscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LbcpBorderHscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LbcpBorderHscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LbpshDisabled => windows_sys::Win32::UI::Controls::LBPSH_DISABLED,
        }
    }
}

/// The state of the `LBCP_BORDER_HVSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LbcpBorderHvscroll {
    /// The default state.
    None,
    /// The `LBPSHV_DISABLED` state.
    LbpshvDisabled,
}

impl Default for LbcpBorderHvscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LbcpBorderHvscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LbcpBorderHvscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LbcpBorderHvscroll {}

impl PartialOrd<i32> for LbcpBorderHvscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LbcpBorderHvscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LbcpBorderHvscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LbcpBorderHvscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LbcpBorderHvscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LbpshvDisabled => windows_sys::Win32::UI::Controls::LBPSHV_DISABLED,
        }
    }
}

/// The state of the `LBCP_BORDER_NOSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LbcpBorderNoscroll {
    /// The default state.
    None,
    /// The `LBPSN_DISABLED` state.
    LbpsnDisabled,
}

impl Default for LbcpBorderNoscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LbcpBorderNoscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LbcpBorderNoscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LbcpBorderNoscroll {}

impl PartialOrd<i32> for LbcpBorderNoscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LbcpBorderNoscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LbcpBorderNoscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LbcpBorderNoscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LbcpBorderNoscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LbpsnDisabled => windows_sys::Win32::UI::Controls::LBPSN_DISABLED,
        }
    }
}

/// The state of the `LBCP_BORDER_VSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LbcpBorderVscroll {
    /// The default state.
    None,
    /// The `LBPSV_DISABLED` state.
    LbpsvDisabled,
}

impl Default for LbcpBorderVscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LbcpBorderVscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LbcpBorderVscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LbcpBorderVscroll {}

impl PartialOrd<i32> for LbcpBorderVscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LbcpBorderVscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LbcpBorderVscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LbcpBorderVscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LbcpBorderVscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LbpsvDisabled => windows_sys::Win32::UI::Controls::LBPSV_DISABLED,
        }
    }
}

/// The state of the `LBCP_ITEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LbcpItem {
    /// The default state.
    None,
    /// The `LBPSI_HOT` state.
    LbpsiHot,
}

impl Default for LbcpItem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LbcpItem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LbcpItem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LbcpItem {}

impl PartialOrd<i32> for LbcpItem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LbcpItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LbcpItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LbcpItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LbcpItem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LbpsiHot => windows_sys::Win32::UI::Controls::LBPSI_HOT,
        }
    }
}

/// The state of the `LVP_COLLAPSEBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpCollapsebutton {
    /// The default state.
    None,
    /// The `LVCB_HOVER` state.
    LvcbHover,
}

impl Default for LvpCollapsebutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpCollapsebutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpCollapsebutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpCollapsebutton {}

impl PartialOrd<i32> for LvpCollapsebutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpCollapsebutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpCollapsebutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpCollapsebutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpCollapsebutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LvcbHover => windows_sys::Win32::UI::Controls::LVCB_HOVER,
        }
    }
}

/// The state of the `LVP_COLUMNDETAIL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpColumndetail {
    /// The default state.
    None,
}

impl Default for LvpColumndetail {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpColumndetail {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpColumndetail {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpColumndetail {}

impl PartialOrd<i32> for LvpColumndetail {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpColumndetail {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpColumndetail {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpColumndetail {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpColumndetail {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `LVP_EMPTYTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpEmptytext {
    /// The default state.
    None,
}

impl Default for LvpEmptytext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpEmptytext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpEmptytext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpEmptytext {}

impl PartialOrd<i32> for LvpEmptytext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpEmptytext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpEmptytext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpEmptytext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpEmptytext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `LVP_EXPANDBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpExpandbutton {
    /// The default state.
    None,
    /// The `LVEB_HOVER` state.
    LvebHover,
}

impl Default for LvpExpandbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpExpandbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpExpandbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpExpandbutton {}

impl PartialOrd<i32> for LvpExpandbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpExpandbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpExpandbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpExpandbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpExpandbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LvebHover => windows_sys::Win32::UI::Controls::LVEB_HOVER,
        }
    }
}

/// The state of the `LVP_GROUPHEADER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpGroupheader {
    /// The default state.
    None,
    /// The `LVGH_CLOSE` state.
    LvghClose,
}

impl Default for LvpGroupheader {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpGroupheader {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpGroupheader {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpGroupheader {}

impl PartialOrd<i32> for LvpGroupheader {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpGroupheader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpGroupheader {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpGroupheader {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpGroupheader {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LvghClose => windows_sys::Win32::UI::Controls::LVGH_CLOSE,
        }
    }
}

/// The state of the `LVP_GROUPHEADERLINE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpGroupheaderline {
    /// The default state.
    None,
    /// The `LVGHL_CLOSE` state.
    LvghlClose,
}

impl Default for LvpGroupheaderline {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpGroupheaderline {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpGroupheaderline {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpGroupheaderline {}

impl PartialOrd<i32> for LvpGroupheaderline {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpGroupheaderline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpGroupheaderline {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpGroupheaderline {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpGroupheaderline {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LvghlClose => windows_sys::Win32::UI::Controls::LVGHL_CLOSE,
        }
    }
}

/// The state of the `LVP_LISTGROUP` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpListgroup {
    /// The default state.
    None,
}

impl Default for LvpListgroup {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpListgroup {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpListgroup {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpListgroup {}

impl PartialOrd<i32> for LvpListgroup {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpListgroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpListgroup {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpListgroup {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpListgroup {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `LVP_LISTDETAIL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpListdetail {
    /// The default state.
    None,
}

impl Default for LvpListdetail {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpListdetail {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpListdetail {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpListdetail {}

impl PartialOrd<i32> for LvpListdetail {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpListdetail {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpListdetail {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpListdetail {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpListdetail {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `LVP_LISTITEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpListitem {
    /// The default state.
    None,
    /// The `LISS_DISABLED` state.
    LissDisabled,
}

impl Default for LvpListitem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpListitem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpListitem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpListitem {}

impl PartialOrd<i32> for LvpListitem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpListitem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpListitem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpListitem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpListitem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::LissDisabled => windows_sys::Win32::UI::Controls::LISS_DISABLED,
        }
    }
}

/// The state of the `LVP_LISTSORTEDDETAIL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum LvpListsorteddetail {
    /// The default state.
    None,
}

impl Default for LvpListsorteddetail {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for LvpListsorteddetail {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for LvpListsorteddetail {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for LvpListsorteddetail {}

impl PartialOrd<i32> for LvpListsorteddetail {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for LvpListsorteddetail {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for LvpListsorteddetail {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for LvpListsorteddetail {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl LvpListsorteddetail {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_BARBACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuBarbackground {
    /// The default state.
    None,
    /// The `MB_ACTIVE` state.
    MbActive,
}

impl Default for MenuBarbackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuBarbackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuBarbackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuBarbackground {}

impl PartialOrd<i32> for MenuBarbackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuBarbackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuBarbackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuBarbackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuBarbackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MbActive => windows_sys::Win32::UI::Controls::MB_ACTIVE,
        }
    }
}

/// The state of the `MENU_BARITEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuBaritem {
    /// The default state.
    None,
    /// The `MBI_DISABLED` state.
    MbiDisabled,
}

impl Default for MenuBaritem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuBaritem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuBaritem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuBaritem {}

impl PartialOrd<i32> for MenuBaritem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuBaritem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuBaritem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuBaritem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuBaritem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MbiDisabled => windows_sys::Win32::UI::Controls::MBI_DISABLED,
        }
    }
}

/// The state of the `MENU_CHEVRON_TMSCHEMA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuChevronTmschema {
    /// The default state.
    None,
}

impl Default for MenuChevronTmschema {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuChevronTmschema {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuChevronTmschema {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuChevronTmschema {}

impl PartialOrd<i32> for MenuChevronTmschema {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuChevronTmschema {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuChevronTmschema {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuChevronTmschema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuChevronTmschema {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_MENUBARDROPDOWN_TMSCHEMA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuMenubardropdownTmschema {
    /// The default state.
    None,
}

impl Default for MenuMenubardropdownTmschema {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuMenubardropdownTmschema {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuMenubardropdownTmschema {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuMenubardropdownTmschema {}

impl PartialOrd<i32> for MenuMenubardropdownTmschema {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuMenubardropdownTmschema {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuMenubardropdownTmschema {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuMenubardropdownTmschema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuMenubardropdownTmschema {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_MENUBARITEM_TMSCHEMA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuMenubaritemTmschema {
    /// The default state.
    None,
}

impl Default for MenuMenubaritemTmschema {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuMenubaritemTmschema {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuMenubaritemTmschema {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuMenubaritemTmschema {}

impl PartialOrd<i32> for MenuMenubaritemTmschema {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuMenubaritemTmschema {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuMenubaritemTmschema {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuMenubaritemTmschema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuMenubaritemTmschema {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_MENUDROPDOWN_TMSCHEMA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuMenudropdownTmschema {
    /// The default state.
    None,
}

impl Default for MenuMenudropdownTmschema {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuMenudropdownTmschema {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuMenudropdownTmschema {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuMenudropdownTmschema {}

impl PartialOrd<i32> for MenuMenudropdownTmschema {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuMenudropdownTmschema {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuMenudropdownTmschema {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuMenudropdownTmschema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuMenudropdownTmschema {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_MENUITEM_TMSCHEMA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuMenuitemTmschema {
    /// The default state.
    None,
}

impl Default for MenuMenuitemTmschema {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuMenuitemTmschema {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuMenuitemTmschema {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuMenuitemTmschema {}

impl PartialOrd<i32> for MenuMenuitemTmschema {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuMenuitemTmschema {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuMenuitemTmschema {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuMenuitemTmschema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuMenuitemTmschema {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_POPUPBACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuPopupbackground {
    /// The default state.
    None,
}

impl Default for MenuPopupbackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuPopupbackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuPopupbackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuPopupbackground {}

impl PartialOrd<i32> for MenuPopupbackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuPopupbackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuPopupbackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuPopupbackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuPopupbackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_POPUPBORDERS` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuPopupborders {
    /// The default state.
    None,
}

impl Default for MenuPopupborders {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuPopupborders {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuPopupborders {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuPopupborders {}

impl PartialOrd<i32> for MenuPopupborders {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuPopupborders {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuPopupborders {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuPopupborders {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuPopupborders {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_POPUPCHECK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuPopupcheck {
    /// The default state.
    None,
    /// The `MC_BULLETDISABLED` state.
    McBulletdisabled,
}

impl Default for MenuPopupcheck {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuPopupcheck {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuPopupcheck {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuPopupcheck {}

impl PartialOrd<i32> for MenuPopupcheck {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuPopupcheck {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuPopupcheck {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuPopupcheck {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuPopupcheck {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::McBulletdisabled => windows_sys::Win32::UI::Controls::MC_BULLETDISABLED,
        }
    }
}

/// The state of the `MENU_POPUPCHECKBACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuPopupcheckbackground {
    /// The default state.
    None,
    /// The `MCB_BITMAP` state.
    McbBitmap,
}

impl Default for MenuPopupcheckbackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuPopupcheckbackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuPopupcheckbackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuPopupcheckbackground {}

impl PartialOrd<i32> for MenuPopupcheckbackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuPopupcheckbackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuPopupcheckbackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuPopupcheckbackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuPopupcheckbackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::McbBitmap => windows_sys::Win32::UI::Controls::MCB_BITMAP,
        }
    }
}

/// The state of the `MENU_POPUPGUTTER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuPopupgutter {
    /// The default state.
    None,
}

impl Default for MenuPopupgutter {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuPopupgutter {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuPopupgutter {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuPopupgutter {}

impl PartialOrd<i32> for MenuPopupgutter {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuPopupgutter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuPopupgutter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuPopupgutter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuPopupgutter {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_POPUPITEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuPopupitem {
    /// The default state.
    None,
    /// The `MPI_DISABLED` state.
    MpiDisabled,
}

impl Default for MenuPopupitem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuPopupitem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuPopupitem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuPopupitem {}

impl PartialOrd<i32> for MenuPopupitem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuPopupitem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuPopupitem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuPopupitem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuPopupitem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MpiDisabled => windows_sys::Win32::UI::Controls::MPI_DISABLED,
        }
    }
}

/// The state of the `MENU_POPUPSEPARATOR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuPopupseparator {
    /// The default state.
    None,
}

impl Default for MenuPopupseparator {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuPopupseparator {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuPopupseparator {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuPopupseparator {}

impl PartialOrd<i32> for MenuPopupseparator {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuPopupseparator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuPopupseparator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuPopupseparator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuPopupseparator {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_POPUPSUBMENU` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuPopupsubmenu {
    /// The default state.
    None,
    /// The `MSM_DISABLED` state.
    MsmDisabled,
}

impl Default for MenuPopupsubmenu {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuPopupsubmenu {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuPopupsubmenu {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuPopupsubmenu {}

impl PartialOrd<i32> for MenuPopupsubmenu {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuPopupsubmenu {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuPopupsubmenu {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuPopupsubmenu {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuPopupsubmenu {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MsmDisabled => windows_sys::Win32::UI::Controls::MSM_DISABLED,
        }
    }
}

/// The state of the `MENU_SEPARATOR_TMSCHEMA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuSeparatorTmschema {
    /// The default state.
    None,
}

impl Default for MenuSeparatorTmschema {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuSeparatorTmschema {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuSeparatorTmschema {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuSeparatorTmschema {}

impl PartialOrd<i32> for MenuSeparatorTmschema {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuSeparatorTmschema {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuSeparatorTmschema {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuSeparatorTmschema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuSeparatorTmschema {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `MENU_SYSTEMCLOSE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuSystemclose {
    /// The default state.
    None,
    /// The `MSYSC_DISABLED` state.
    MsyscDisabled,
}

impl Default for MenuSystemclose {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuSystemclose {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuSystemclose {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuSystemclose {}

impl PartialOrd<i32> for MenuSystemclose {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuSystemclose {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuSystemclose {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuSystemclose {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuSystemclose {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MsyscDisabled => windows_sys::Win32::UI::Controls::MSYSC_DISABLED,
        }
    }
}

/// The state of the `MENU_SYSTEMMAXIMIZE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuSystemmaximize {
    /// The default state.
    None,
    /// The `MSYSMX_DISABLED` state.
    MsysmxDisabled,
}

impl Default for MenuSystemmaximize {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuSystemmaximize {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuSystemmaximize {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuSystemmaximize {}

impl PartialOrd<i32> for MenuSystemmaximize {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuSystemmaximize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuSystemmaximize {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuSystemmaximize {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuSystemmaximize {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MsysmxDisabled => windows_sys::Win32::UI::Controls::MSYSMX_DISABLED,
        }
    }
}

/// The state of the `MENU_SYSTEMMINIMIZE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuSystemminimize {
    /// The default state.
    None,
    /// The `MSYSMN_DISABLED` state.
    MsysmnDisabled,
}

impl Default for MenuSystemminimize {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuSystemminimize {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuSystemminimize {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuSystemminimize {}

impl PartialOrd<i32> for MenuSystemminimize {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuSystemminimize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuSystemminimize {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuSystemminimize {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuSystemminimize {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MsysmnDisabled => windows_sys::Win32::UI::Controls::MSYSMN_DISABLED,
        }
    }
}

/// The state of the `MENU_SYSTEMRESTORE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MenuSystemrestore {
    /// The default state.
    None,
    /// The `MSYSR_DISABLED` state.
    MsysrDisabled,
}

impl Default for MenuSystemrestore {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MenuSystemrestore {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MenuSystemrestore {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MenuSystemrestore {}

impl PartialOrd<i32> for MenuSystemrestore {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MenuSystemrestore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MenuSystemrestore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MenuSystemrestore {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MenuSystemrestore {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MsysrDisabled => windows_sys::Win32::UI::Controls::MSYSR_DISABLED,
        }
    }
}

/// The state of the `MDP_NEWAPPBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MdpNewappbutton {
    /// The default state.
    None,
    /// The `MDS_CHECKED` state.
    MdsChecked,
}

impl Default for MdpNewappbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MdpNewappbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MdpNewappbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MdpNewappbutton {}

impl PartialOrd<i32> for MdpNewappbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MdpNewappbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MdpNewappbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MdpNewappbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MdpNewappbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MdsChecked => windows_sys::Win32::UI::Controls::MDS_CHECKED,
        }
    }
}

/// The state of the `MDP_SEPERATOR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum MdpSeperator {
    /// The default state.
    None,
}

impl Default for MdpSeperator {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for MdpSeperator {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for MdpSeperator {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for MdpSeperator {}

impl PartialOrd<i32> for MdpSeperator {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for MdpSeperator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for MdpSeperator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for MdpSeperator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl MdpSeperator {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `NAV_BACKBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum NavBackbutton {
    /// The default state.
    None,
    /// The `NAV_BB_DISABLED` state.
    NavBbDisabled,
}

impl Default for NavBackbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for NavBackbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for NavBackbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for NavBackbutton {}

impl PartialOrd<i32> for NavBackbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for NavBackbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for NavBackbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for NavBackbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl NavBackbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::NavBbDisabled => windows_sys::Win32::UI::Controls::NAV_BB_DISABLED,
        }
    }
}

/// The state of the `NAV_FORWARDBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum NavForwardbutton {
    /// The default state.
    None,
    /// The `NAV_FB_DISABLED` state.
    NavFbDisabled,
}

impl Default for NavForwardbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for NavForwardbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for NavForwardbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for NavForwardbutton {}

impl PartialOrd<i32> for NavForwardbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for NavForwardbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for NavForwardbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for NavForwardbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl NavForwardbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::NavFbDisabled => windows_sys::Win32::UI::Controls::NAV_FB_DISABLED,
        }
    }
}

/// The state of the `NAV_MENUBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum NavMenubutton {
    /// The default state.
    None,
    /// The `NAV_MB_DISABLED` state.
    NavMbDisabled,
}

impl Default for NavMenubutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for NavMenubutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for NavMenubutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for NavMenubutton {}

impl PartialOrd<i32> for NavMenubutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for NavMenubutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for NavMenubutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for NavMenubutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl NavMenubutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::NavMbDisabled => windows_sys::Win32::UI::Controls::NAV_MB_DISABLED,
        }
    }
}

/// The state of the `PGRP_DOWN` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PgrpDown {
    /// The default state.
    None,
    /// The `DNS_DISABLED` state.
    DnsDisabled,
}

impl Default for PgrpDown {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PgrpDown {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PgrpDown {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PgrpDown {}

impl PartialOrd<i32> for PgrpDown {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PgrpDown {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PgrpDown {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PgrpDown {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PgrpDown {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DnsDisabled => windows_sys::Win32::UI::Controls::DNS_DISABLED,
        }
    }
}

/// The state of the `PGRP_DOWNHORZ` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PgrpDownhorz {
    /// The default state.
    None,
    /// The `DNHZS_DISABLED` state.
    DnhzsDisabled,
}

impl Default for PgrpDownhorz {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PgrpDownhorz {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PgrpDownhorz {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PgrpDownhorz {}

impl PartialOrd<i32> for PgrpDownhorz {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PgrpDownhorz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PgrpDownhorz {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PgrpDownhorz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PgrpDownhorz {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DnhzsDisabled => windows_sys::Win32::UI::Controls::DNHZS_DISABLED,
        }
    }
}

/// The state of the `PGRP_UP` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PgrpUp {
    /// The default state.
    None,
    /// The `UPS_DISABLED` state.
    UpsDisabled,
}

impl Default for PgrpUp {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PgrpUp {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PgrpUp {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PgrpUp {}

impl PartialOrd<i32> for PgrpUp {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PgrpUp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PgrpUp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PgrpUp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PgrpUp {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::UpsDisabled => windows_sys::Win32::UI::Controls::UPS_DISABLED,
        }
    }
}

/// The state of the `PGRP_UPHORZ` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PgrpUphorz {
    /// The default state.
    None,
    /// The `UPHZS_DISABLED` state.
    UphzsDisabled,
}

impl Default for PgrpUphorz {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PgrpUphorz {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PgrpUphorz {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PgrpUphorz {}

impl PartialOrd<i32> for PgrpUphorz {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PgrpUphorz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PgrpUphorz {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PgrpUphorz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PgrpUphorz {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::UphzsDisabled => windows_sys::Win32::UI::Controls::UPHZS_DISABLED,
        }
    }
}

/// The state of the `PP_BAR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpBar {
    /// The default state.
    None,
}

impl Default for PpBar {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpBar {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpBar {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpBar {}

impl PartialOrd<i32> for PpBar {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpBar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpBar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpBar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpBar {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `PP_BARVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpBarvert {
    /// The default state.
    None,
}

impl Default for PpBarvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpBarvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpBarvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpBarvert {}

impl PartialOrd<i32> for PpBarvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpBarvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpBarvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpBarvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpBarvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `PP_CHUNK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpChunk {
    /// The default state.
    None,
}

impl Default for PpChunk {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpChunk {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpChunk {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpChunk {}

impl PartialOrd<i32> for PpChunk {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpChunk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpChunk {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpChunk {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpChunk {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `PP_CHUNKVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpChunkvert {
    /// The default state.
    None,
}

impl Default for PpChunkvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpChunkvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpChunkvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpChunkvert {}

impl PartialOrd<i32> for PpChunkvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpChunkvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpChunkvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpChunkvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpChunkvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `PP_FILL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpFill {
    /// The default state.
    None,
    /// The `PBFS_ERROR` state.
    PbfsError,
}

impl Default for PpFill {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpFill {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpFill {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpFill {}

impl PartialOrd<i32> for PpFill {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpFill {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpFill {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpFill {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpFill {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::PbfsError => windows_sys::Win32::UI::Controls::PBFS_ERROR,
        }
    }
}

/// The state of the `PP_FILLVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpFillvert {
    /// The default state.
    None,
    /// The `PBFVS_ERROR` state.
    PbfvsError,
}

impl Default for PpFillvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpFillvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpFillvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpFillvert {}

impl PartialOrd<i32> for PpFillvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpFillvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpFillvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpFillvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpFillvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::PbfvsError => windows_sys::Win32::UI::Controls::PBFVS_ERROR,
        }
    }
}

/// The state of the `PP_MOVEOVERLAY` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpMoveoverlay {
    /// The default state.
    None,
}

impl Default for PpMoveoverlay {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpMoveoverlay {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpMoveoverlay {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpMoveoverlay {}

impl PartialOrd<i32> for PpMoveoverlay {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpMoveoverlay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpMoveoverlay {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpMoveoverlay {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpMoveoverlay {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `PP_MOVEOVERLAYVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpMoveoverlayvert {
    /// The default state.
    None,
}

impl Default for PpMoveoverlayvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpMoveoverlayvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpMoveoverlayvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpMoveoverlayvert {}

impl PartialOrd<i32> for PpMoveoverlayvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpMoveoverlayvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpMoveoverlayvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpMoveoverlayvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpMoveoverlayvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `PP_PULSEOVERLAY` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpPulseoverlay {
    /// The default state.
    None,
}

impl Default for PpPulseoverlay {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpPulseoverlay {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpPulseoverlay {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpPulseoverlay {}

impl PartialOrd<i32> for PpPulseoverlay {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpPulseoverlay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpPulseoverlay {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpPulseoverlay {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpPulseoverlay {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `PP_PULSEOVERLAYVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpPulseoverlayvert {
    /// The default state.
    None,
}

impl Default for PpPulseoverlayvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpPulseoverlayvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpPulseoverlayvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpPulseoverlayvert {}

impl PartialOrd<i32> for PpPulseoverlayvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpPulseoverlayvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpPulseoverlayvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpPulseoverlayvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpPulseoverlayvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `PP_TRANSPARENTBAR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpTransparentbar {
    /// The default state.
    None,
    /// The `PBBS_NORMAL` state.
    PbbsNormal,
}

impl Default for PpTransparentbar {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpTransparentbar {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpTransparentbar {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpTransparentbar {}

impl PartialOrd<i32> for PpTransparentbar {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpTransparentbar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpTransparentbar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpTransparentbar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpTransparentbar {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::PbbsNormal => windows_sys::Win32::UI::Controls::PBBS_NORMAL,
        }
    }
}

/// The state of the `PP_TRANSPARENTBARVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PpTransparentbarvert {
    /// The default state.
    None,
    /// The `PBBVS_NORMAL` state.
    PbbvsNormal,
}

impl Default for PpTransparentbarvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for PpTransparentbarvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for PpTransparentbarvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for PpTransparentbarvert {}

impl PartialOrd<i32> for PpTransparentbarvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for PpTransparentbarvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for PpTransparentbarvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for PpTransparentbarvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl PpTransparentbarvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::PbbvsNormal => windows_sys::Win32::UI::Controls::PBBVS_NORMAL,
        }
    }
}

/// The state of the `RP_BACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum RpBackground {
    /// The default state.
    None,
}

impl Default for RpBackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for RpBackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for RpBackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for RpBackground {}

impl PartialOrd<i32> for RpBackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for RpBackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for RpBackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for RpBackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl RpBackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `RP_BAND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum RpBand {
    /// The default state.
    None,
}

impl Default for RpBand {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for RpBand {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for RpBand {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for RpBand {}

impl PartialOrd<i32> for RpBand {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for RpBand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for RpBand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for RpBand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl RpBand {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `RP_CHEVRON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum RpChevron {
    /// The default state.
    None,
    /// The `CHEVS_HOT` state.
    ChevsHot,
}

impl Default for RpChevron {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for RpChevron {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for RpChevron {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for RpChevron {}

impl PartialOrd<i32> for RpChevron {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for RpChevron {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for RpChevron {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for RpChevron {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl RpChevron {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ChevsHot => windows_sys::Win32::UI::Controls::CHEVS_HOT,
        }
    }
}

/// The state of the `RP_CHEVRONVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum RpChevronvert {
    /// The default state.
    None,
    /// The `CHEVSV_HOT` state.
    ChevsvHot,
}

impl Default for RpChevronvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for RpChevronvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for RpChevronvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for RpChevronvert {}

impl PartialOrd<i32> for RpChevronvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for RpChevronvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for RpChevronvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for RpChevronvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl RpChevronvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ChevsvHot => windows_sys::Win32::UI::Controls::CHEVSV_HOT,
        }
    }
}

/// The state of the `RP_GRIPPER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum RpGripper {
    /// The default state.
    None,
}

impl Default for RpGripper {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for RpGripper {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for RpGripper {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for RpGripper {}

impl PartialOrd<i32> for RpGripper {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for RpGripper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for RpGripper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for RpGripper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl RpGripper {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `RP_GRIPPERVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum RpGrippervert {
    /// The default state.
    None,
}

impl Default for RpGrippervert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for RpGrippervert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for RpGrippervert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for RpGrippervert {}

impl PartialOrd<i32> for RpGrippervert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for RpGrippervert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for RpGrippervert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for RpGrippervert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl RpGrippervert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `RP_SPLITTER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum RpSplitter {
    /// The default state.
    None,
    /// The `SPLITS_HOT` state.
    SplitsHot,
}

impl Default for RpSplitter {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for RpSplitter {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for RpSplitter {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for RpSplitter {}

impl PartialOrd<i32> for RpSplitter {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for RpSplitter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for RpSplitter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for RpSplitter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl RpSplitter {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::SplitsHot => windows_sys::Win32::UI::Controls::SPLITS_HOT,
        }
    }
}

/// The state of the `RP_SPLITTERVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum RpSplittervert {
    /// The default state.
    None,
    /// The `SPLITSV_HOT` state.
    SplitsvHot,
}

impl Default for RpSplittervert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for RpSplittervert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for RpSplittervert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for RpSplittervert {}

impl PartialOrd<i32> for RpSplittervert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for RpSplittervert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for RpSplittervert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for RpSplittervert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl RpSplittervert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::SplitsvHot => windows_sys::Win32::UI::Controls::SPLITSV_HOT,
        }
    }
}

/// The state of the `SBP_ARROWBTN` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpArrowbtn {
    /// The default state.
    None,
    /// The `ABS_DOWNDISABLED` state.
    AbsDowndisabled,
}

impl Default for SbpArrowbtn {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpArrowbtn {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpArrowbtn {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpArrowbtn {}

impl PartialOrd<i32> for SbpArrowbtn {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpArrowbtn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpArrowbtn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpArrowbtn {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpArrowbtn {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::AbsDowndisabled => windows_sys::Win32::UI::Controls::ABS_DOWNDISABLED,
        }
    }
}

/// The state of the `SBP_GRIPPERHORZ` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpGripperhorz {
    /// The default state.
    None,
    /// The `SCRBS_DISABLED` state.
    ScrbsDisabled,
}

impl Default for SbpGripperhorz {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpGripperhorz {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpGripperhorz {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpGripperhorz {}

impl PartialOrd<i32> for SbpGripperhorz {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpGripperhorz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpGripperhorz {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpGripperhorz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpGripperhorz {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ScrbsDisabled => windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
        }
    }
}

/// The state of the `SBP_GRIPPERVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpGrippervert {
    /// The default state.
    None,
    /// The `SCRBS_DISABLED` state.
    ScrbsDisabled,
}

impl Default for SbpGrippervert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpGrippervert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpGrippervert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpGrippervert {}

impl PartialOrd<i32> for SbpGrippervert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpGrippervert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpGrippervert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpGrippervert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpGrippervert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ScrbsDisabled => windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
        }
    }
}

/// The state of the `SBP_LOWERTRACKHORZ` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpLowertrackhorz {
    /// The default state.
    None,
    /// The `SCRBS_DISABLED` state.
    ScrbsDisabled,
}

impl Default for SbpLowertrackhorz {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpLowertrackhorz {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpLowertrackhorz {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpLowertrackhorz {}

impl PartialOrd<i32> for SbpLowertrackhorz {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpLowertrackhorz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpLowertrackhorz {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpLowertrackhorz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpLowertrackhorz {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ScrbsDisabled => windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
        }
    }
}

/// The state of the `SBP_LOWERTRACKVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpLowertrackvert {
    /// The default state.
    None,
    /// The `SCRBS_DISABLED` state.
    ScrbsDisabled,
}

impl Default for SbpLowertrackvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpLowertrackvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpLowertrackvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpLowertrackvert {}

impl PartialOrd<i32> for SbpLowertrackvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpLowertrackvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpLowertrackvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpLowertrackvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpLowertrackvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ScrbsDisabled => windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
        }
    }
}

/// The state of the `SBP_SIZEBOX` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpSizebox {
    /// The default state.
    None,
    /// The `SZB_HALFBOTTOMRIGHTALIGN` state.
    SzbHalfbottomrightalign,
}

impl Default for SbpSizebox {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpSizebox {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpSizebox {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpSizebox {}

impl PartialOrd<i32> for SbpSizebox {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpSizebox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpSizebox {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpSizebox {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpSizebox {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::SzbHalfbottomrightalign => windows_sys::Win32::UI::Controls::SZB_HALFBOTTOMRIGHTALIGN,
        }
    }
}

/// The state of the `SBP_THUMBBTNHORZ` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpThumbbtnhorz {
    /// The default state.
    None,
    /// The `SCRBS_DISABLED` state.
    ScrbsDisabled,
}

impl Default for SbpThumbbtnhorz {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpThumbbtnhorz {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpThumbbtnhorz {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpThumbbtnhorz {}

impl PartialOrd<i32> for SbpThumbbtnhorz {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpThumbbtnhorz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpThumbbtnhorz {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpThumbbtnhorz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpThumbbtnhorz {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ScrbsDisabled => windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
        }
    }
}

/// The state of the `SBP_THUMBBTNVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpThumbbtnvert {
    /// The default state.
    None,
    /// The `SCRBS_DISABLED` state.
    ScrbsDisabled,
}

impl Default for SbpThumbbtnvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpThumbbtnvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpThumbbtnvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpThumbbtnvert {}

impl PartialOrd<i32> for SbpThumbbtnvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpThumbbtnvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpThumbbtnvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpThumbbtnvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpThumbbtnvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ScrbsDisabled => windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
        }
    }
}

/// The state of the `SBP_UPPERTRACKHORZ` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpUppertrackhorz {
    /// The default state.
    None,
    /// The `SCRBS_DISABLED` state.
    ScrbsDisabled,
}

impl Default for SbpUppertrackhorz {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpUppertrackhorz {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpUppertrackhorz {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpUppertrackhorz {}

impl PartialOrd<i32> for SbpUppertrackhorz {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpUppertrackhorz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpUppertrackhorz {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpUppertrackhorz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpUppertrackhorz {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ScrbsDisabled => windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
        }
    }
}

/// The state of the `SBP_UPPERTRACKVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SbpUppertrackvert {
    /// The default state.
    None,
    /// The `SCRBS_DISABLED` state.
    ScrbsDisabled,
}

impl Default for SbpUppertrackvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SbpUppertrackvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SbpUppertrackvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SbpUppertrackvert {}

impl PartialOrd<i32> for SbpUppertrackvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SbpUppertrackvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SbpUppertrackvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SbpUppertrackvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SbpUppertrackvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ScrbsDisabled => windows_sys::Win32::UI::Controls::SCRBS_DISABLED,
        }
    }
}

/// The state of the `SPNP_DOWN` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SpnpDown {
    /// The default state.
    None,
    /// The `DNS_DISABLED` state.
    DnsDisabled,
}

impl Default for SpnpDown {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SpnpDown {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SpnpDown {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SpnpDown {}

impl PartialOrd<i32> for SpnpDown {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SpnpDown {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SpnpDown {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SpnpDown {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SpnpDown {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DnsDisabled => windows_sys::Win32::UI::Controls::DNS_DISABLED,
        }
    }
}

/// The state of the `SPNP_DOWNHORZ` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SpnpDownhorz {
    /// The default state.
    None,
    /// The `DNHZS_DISABLED` state.
    DnhzsDisabled,
}

impl Default for SpnpDownhorz {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SpnpDownhorz {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SpnpDownhorz {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SpnpDownhorz {}

impl PartialOrd<i32> for SpnpDownhorz {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SpnpDownhorz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SpnpDownhorz {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SpnpDownhorz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SpnpDownhorz {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::DnhzsDisabled => windows_sys::Win32::UI::Controls::DNHZS_DISABLED,
        }
    }
}

/// The state of the `SPNP_UP` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SpnpUp {
    /// The default state.
    None,
    /// The `UPS_DISABLED` state.
    UpsDisabled,
}

impl Default for SpnpUp {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SpnpUp {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SpnpUp {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SpnpUp {}

impl PartialOrd<i32> for SpnpUp {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SpnpUp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SpnpUp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SpnpUp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SpnpUp {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::UpsDisabled => windows_sys::Win32::UI::Controls::UPS_DISABLED,
        }
    }
}

/// The state of the `SPNP_UPHORZ` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SpnpUphorz {
    /// The default state.
    None,
    /// The `UPHZS_DISABLED` state.
    UphzsDisabled,
}

impl Default for SpnpUphorz {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SpnpUphorz {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SpnpUphorz {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SpnpUphorz {}

impl PartialOrd<i32> for SpnpUphorz {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SpnpUphorz {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SpnpUphorz {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SpnpUphorz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SpnpUphorz {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::UphzsDisabled => windows_sys::Win32::UI::Controls::UPHZS_DISABLED,
        }
    }
}

/// The state of the `SPP_LOGOFF` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppLogoff {
    /// The default state.
    None,
}

impl Default for SppLogoff {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppLogoff {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppLogoff {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppLogoff {}

impl PartialOrd<i32> for SppLogoff {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppLogoff {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppLogoff {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppLogoff {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppLogoff {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SPP_LOGOFFBUTTONS` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppLogoffbuttons {
    /// The default state.
    None,
    /// The `SPLS_HOT` state.
    SplsHot,
}

impl Default for SppLogoffbuttons {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppLogoffbuttons {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppLogoffbuttons {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppLogoffbuttons {}

impl PartialOrd<i32> for SppLogoffbuttons {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppLogoffbuttons {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppLogoffbuttons {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppLogoffbuttons {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppLogoffbuttons {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::SplsHot => windows_sys::Win32::UI::Controls::SPLS_HOT,
        }
    }
}

/// The state of the `SPP_MOREPROGRAMS` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppMoreprograms {
    /// The default state.
    None,
}

impl Default for SppMoreprograms {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppMoreprograms {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppMoreprograms {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppMoreprograms {}

impl PartialOrd<i32> for SppMoreprograms {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppMoreprograms {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppMoreprograms {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppMoreprograms {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppMoreprograms {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SPP_MOREPROGRAMSARROW` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppMoreprogramsarrow {
    /// The default state.
    None,
    /// The `SPS_HOT` state.
    SpsHot,
}

impl Default for SppMoreprogramsarrow {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppMoreprogramsarrow {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppMoreprogramsarrow {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppMoreprogramsarrow {}

impl PartialOrd<i32> for SppMoreprogramsarrow {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppMoreprogramsarrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppMoreprogramsarrow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppMoreprogramsarrow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppMoreprogramsarrow {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::SpsHot => windows_sys::Win32::UI::Controls::SPS_HOT,
        }
    }
}

/// The state of the `SPP_PLACESLIST` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppPlaceslist {
    /// The default state.
    None,
}

impl Default for SppPlaceslist {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppPlaceslist {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppPlaceslist {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppPlaceslist {}

impl PartialOrd<i32> for SppPlaceslist {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppPlaceslist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppPlaceslist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppPlaceslist {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppPlaceslist {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SPP_PLACESLISTSEPARATOR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppPlaceslistseparator {
    /// The default state.
    None,
}

impl Default for SppPlaceslistseparator {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppPlaceslistseparator {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppPlaceslistseparator {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppPlaceslistseparator {}

impl PartialOrd<i32> for SppPlaceslistseparator {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppPlaceslistseparator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppPlaceslistseparator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppPlaceslistseparator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppPlaceslistseparator {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SPP_PREVIEW` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppPreview {
    /// The default state.
    None,
}

impl Default for SppPreview {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppPreview {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppPreview {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppPreview {}

impl PartialOrd<i32> for SppPreview {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppPreview {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppPreview {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppPreview {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppPreview {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SPP_PROGLIST` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppProglist {
    /// The default state.
    None,
}

impl Default for SppProglist {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppProglist {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppProglist {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppProglist {}

impl PartialOrd<i32> for SppProglist {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppProglist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppProglist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppProglist {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppProglist {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SPP_PROGLISTSEPARATOR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppProglistseparator {
    /// The default state.
    None,
}

impl Default for SppProglistseparator {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppProglistseparator {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppProglistseparator {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppProglistseparator {}

impl PartialOrd<i32> for SppProglistseparator {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppProglistseparator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppProglistseparator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppProglistseparator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppProglistseparator {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SPP_USERPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppUserpane {
    /// The default state.
    None,
}

impl Default for SppUserpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppUserpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppUserpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppUserpane {}

impl PartialOrd<i32> for SppUserpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppUserpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppUserpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppUserpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppUserpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SPP_USERPICTURE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SppUserpicture {
    /// The default state.
    None,
}

impl Default for SppUserpicture {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SppUserpicture {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SppUserpicture {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SppUserpicture {}

impl PartialOrd<i32> for SppUserpicture {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SppUserpicture {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SppUserpicture {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SppUserpicture {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SppUserpicture {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SP_GRIPPER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SpGripper {
    /// The default state.
    None,
}

impl Default for SpGripper {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SpGripper {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SpGripper {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SpGripper {}

impl PartialOrd<i32> for SpGripper {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SpGripper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SpGripper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SpGripper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SpGripper {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SP_GRIPPERPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SpGripperpane {
    /// The default state.
    None,
}

impl Default for SpGripperpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SpGripperpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SpGripperpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SpGripperpane {}

impl PartialOrd<i32> for SpGripperpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SpGripperpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SpGripperpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SpGripperpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SpGripperpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `SP_PANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum SpPane {
    /// The default state.
    None,
}

impl Default for SpPane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for SpPane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for SpPane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for SpPane {}

impl PartialOrd<i32> for SpPane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for SpPane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for SpPane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for SpPane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl SpPane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TABP_AEROWIZARDBODY` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpAerowizardbody {
    /// The default state.
    None,
}

impl Default for TabpAerowizardbody {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpAerowizardbody {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpAerowizardbody {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpAerowizardbody {}

impl PartialOrd<i32> for TabpAerowizardbody {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpAerowizardbody {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpAerowizardbody {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpAerowizardbody {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpAerowizardbody {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TABP_BODY` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpBody {
    /// The default state.
    None,
}

impl Default for TabpBody {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpBody {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpBody {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpBody {}

impl PartialOrd<i32> for TabpBody {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpBody {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpBody {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpBody {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpBody {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TABP_PANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpPane {
    /// The default state.
    None,
}

impl Default for TabpPane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpPane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpPane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpPane {}

impl PartialOrd<i32> for TabpPane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpPane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpPane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpPane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpPane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TABP_TABITEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpTabitem {
    /// The default state.
    None,
    /// The `TIS_DISABLED` state.
    TisDisabled,
}

impl Default for TabpTabitem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpTabitem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpTabitem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpTabitem {}

impl PartialOrd<i32> for TabpTabitem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpTabitem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpTabitem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpTabitem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpTabitem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TisDisabled => windows_sys::Win32::UI::Controls::TIS_DISABLED,
        }
    }
}

/// The state of the `TABP_TABITEMBOTHEDGE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpTabitembothedge {
    /// The default state.
    None,
    /// The `TIBES_DISABLED` state.
    TibesDisabled,
}

impl Default for TabpTabitembothedge {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpTabitembothedge {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpTabitembothedge {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpTabitembothedge {}

impl PartialOrd<i32> for TabpTabitembothedge {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpTabitembothedge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpTabitembothedge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpTabitembothedge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpTabitembothedge {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TibesDisabled => windows_sys::Win32::UI::Controls::TIBES_DISABLED,
        }
    }
}

/// The state of the `TABP_TABITEMLEFTEDGE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpTabitemleftedge {
    /// The default state.
    None,
    /// The `TILES_DISABLED` state.
    TilesDisabled,
}

impl Default for TabpTabitemleftedge {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpTabitemleftedge {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpTabitemleftedge {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpTabitemleftedge {}

impl PartialOrd<i32> for TabpTabitemleftedge {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpTabitemleftedge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpTabitemleftedge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpTabitemleftedge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpTabitemleftedge {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TilesDisabled => windows_sys::Win32::UI::Controls::TILES_DISABLED,
        }
    }
}

/// The state of the `TABP_TABITEMRIGHTEDGE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpTabitemrightedge {
    /// The default state.
    None,
    /// The `TIRES_DISABLED` state.
    TiresDisabled,
}

impl Default for TabpTabitemrightedge {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpTabitemrightedge {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpTabitemrightedge {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpTabitemrightedge {}

impl PartialOrd<i32> for TabpTabitemrightedge {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpTabitemrightedge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpTabitemrightedge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpTabitemrightedge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpTabitemrightedge {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TiresDisabled => windows_sys::Win32::UI::Controls::TIRES_DISABLED,
        }
    }
}

/// The state of the `TABP_TOPTABITEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpToptabitem {
    /// The default state.
    None,
    /// The `TTIS_DISABLED` state.
    TtisDisabled,
}

impl Default for TabpToptabitem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpToptabitem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpToptabitem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpToptabitem {}

impl PartialOrd<i32> for TabpToptabitem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpToptabitem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpToptabitem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpToptabitem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpToptabitem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtisDisabled => windows_sys::Win32::UI::Controls::TTIS_DISABLED,
        }
    }
}

/// The state of the `TABP_TOPTABITEMBOTHEDGE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpToptabitembothedge {
    /// The default state.
    None,
    /// The `TTIBES_DISABLED` state.
    TtibesDisabled,
}

impl Default for TabpToptabitembothedge {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpToptabitembothedge {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpToptabitembothedge {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpToptabitembothedge {}

impl PartialOrd<i32> for TabpToptabitembothedge {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpToptabitembothedge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpToptabitembothedge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpToptabitembothedge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpToptabitembothedge {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtibesDisabled => windows_sys::Win32::UI::Controls::TTIBES_DISABLED,
        }
    }
}

/// The state of the `TABP_TOPTABITEMLEFTEDGE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpToptabitemleftedge {
    /// The default state.
    None,
    /// The `TTILES_DISABLED` state.
    TtilesDisabled,
}

impl Default for TabpToptabitemleftedge {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpToptabitemleftedge {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpToptabitemleftedge {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpToptabitemleftedge {}

impl PartialOrd<i32> for TabpToptabitemleftedge {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpToptabitemleftedge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpToptabitemleftedge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpToptabitemleftedge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpToptabitemleftedge {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtilesDisabled => windows_sys::Win32::UI::Controls::TTILES_DISABLED,
        }
    }
}

/// The state of the `TABP_TOPTABITEMRIGHTEDGE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TabpToptabitemrightedge {
    /// The default state.
    None,
    /// The `TTIRES_DISABLED` state.
    TtiresDisabled,
}

impl Default for TabpToptabitemrightedge {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TabpToptabitemrightedge {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TabpToptabitemrightedge {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TabpToptabitemrightedge {}

impl PartialOrd<i32> for TabpToptabitemrightedge {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TabpToptabitemrightedge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TabpToptabitemrightedge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TabpToptabitemrightedge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TabpToptabitemrightedge {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtiresDisabled => windows_sys::Win32::UI::Controls::TTIRES_DISABLED,
        }
    }
}

/// The state of the `TDP_GROUPCOUNT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdpGroupcount {
    /// The default state.
    None,
}

impl Default for TdpGroupcount {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdpGroupcount {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdpGroupcount {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdpGroupcount {}

impl PartialOrd<i32> for TdpGroupcount {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdpGroupcount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdpGroupcount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdpGroupcount {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdpGroupcount {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDP_FLASHBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdpFlashbutton {
    /// The default state.
    None,
}

impl Default for TdpFlashbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdpFlashbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdpFlashbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdpFlashbutton {}

impl PartialOrd<i32> for TdpFlashbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdpFlashbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdpFlashbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdpFlashbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdpFlashbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDP_FLASHBUTTONGROUPMENU` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdpFlashbuttongroupmenu {
    /// The default state.
    None,
}

impl Default for TdpFlashbuttongroupmenu {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdpFlashbuttongroupmenu {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdpFlashbuttongroupmenu {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdpFlashbuttongroupmenu {}

impl PartialOrd<i32> for TdpFlashbuttongroupmenu {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdpFlashbuttongroupmenu {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdpFlashbuttongroupmenu {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdpFlashbuttongroupmenu {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdpFlashbuttongroupmenu {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TBP_BACKGROUNDBOTTOM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TbpBackgroundbottom {
    /// The default state.
    None,
}

impl Default for TbpBackgroundbottom {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TbpBackgroundbottom {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TbpBackgroundbottom {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TbpBackgroundbottom {}

impl PartialOrd<i32> for TbpBackgroundbottom {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TbpBackgroundbottom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TbpBackgroundbottom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TbpBackgroundbottom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TbpBackgroundbottom {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TBP_BACKGROUNDLEFT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TbpBackgroundleft {
    /// The default state.
    None,
}

impl Default for TbpBackgroundleft {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TbpBackgroundleft {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TbpBackgroundleft {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TbpBackgroundleft {}

impl PartialOrd<i32> for TbpBackgroundleft {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TbpBackgroundleft {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TbpBackgroundleft {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TbpBackgroundleft {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TbpBackgroundleft {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TBP_BACKGROUNDRIGHT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TbpBackgroundright {
    /// The default state.
    None,
}

impl Default for TbpBackgroundright {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TbpBackgroundright {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TbpBackgroundright {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TbpBackgroundright {}

impl PartialOrd<i32> for TbpBackgroundright {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TbpBackgroundright {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TbpBackgroundright {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TbpBackgroundright {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TbpBackgroundright {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TBP_BACKGROUNDTOP` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TbpBackgroundtop {
    /// The default state.
    None,
}

impl Default for TbpBackgroundtop {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TbpBackgroundtop {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TbpBackgroundtop {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TbpBackgroundtop {}

impl PartialOrd<i32> for TbpBackgroundtop {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TbpBackgroundtop {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TbpBackgroundtop {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TbpBackgroundtop {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TbpBackgroundtop {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TBP_SIZINGBARBOTTOM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TbpSizingbarbottom {
    /// The default state.
    None,
}

impl Default for TbpSizingbarbottom {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TbpSizingbarbottom {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TbpSizingbarbottom {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TbpSizingbarbottom {}

impl PartialOrd<i32> for TbpSizingbarbottom {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TbpSizingbarbottom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TbpSizingbarbottom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TbpSizingbarbottom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TbpSizingbarbottom {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TBP_SIZINGBARRIGHT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TbpSizingbarright {
    /// The default state.
    None,
}

impl Default for TbpSizingbarright {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TbpSizingbarright {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TbpSizingbarright {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TbpSizingbarright {}

impl PartialOrd<i32> for TbpSizingbarright {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TbpSizingbarright {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TbpSizingbarright {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TbpSizingbarright {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TbpSizingbarright {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TBP_SIZINGBARTOP` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TbpSizingbartop {
    /// The default state.
    None,
}

impl Default for TbpSizingbartop {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TbpSizingbartop {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TbpSizingbartop {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TbpSizingbartop {}

impl PartialOrd<i32> for TbpSizingbartop {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TbpSizingbartop {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TbpSizingbartop {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TbpSizingbartop {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TbpSizingbartop {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_BUTTONSECTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgButtonsection {
    /// The default state.
    None,
}

impl Default for TdlgButtonsection {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgButtonsection {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgButtonsection {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgButtonsection {}

impl PartialOrd<i32> for TdlgButtonsection {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgButtonsection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgButtonsection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgButtonsection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgButtonsection {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_BUTTONWRAPPER` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgButtonwrapper {
    /// The default state.
    None,
}

impl Default for TdlgButtonwrapper {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgButtonwrapper {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgButtonwrapper {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgButtonwrapper {}

impl PartialOrd<i32> for TdlgButtonwrapper {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgButtonwrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgButtonwrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgButtonwrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgButtonwrapper {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_COMMANDLINKPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgCommandlinkpane {
    /// The default state.
    None,
}

impl Default for TdlgCommandlinkpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgCommandlinkpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgCommandlinkpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgCommandlinkpane {}

impl PartialOrd<i32> for TdlgCommandlinkpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgCommandlinkpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgCommandlinkpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgCommandlinkpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgCommandlinkpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_CONTENTICON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgContenticon {
    /// The default state.
    None,
}

impl Default for TdlgContenticon {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgContenticon {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgContenticon {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgContenticon {}

impl PartialOrd<i32> for TdlgContenticon {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgContenticon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgContenticon {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgContenticon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgContenticon {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_CONTENTPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgContentpane {
    /// The default state.
    None,
    /// The `TDLGCPS_STANDALONE` state.
    TdlgcpsStandalone,
}

impl Default for TdlgContentpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgContentpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgContentpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgContentpane {}

impl PartialOrd<i32> for TdlgContentpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgContentpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgContentpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgContentpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgContentpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TdlgcpsStandalone => windows_sys::Win32::UI::Controls::TDLGCPS_STANDALONE,
        }
    }
}

/// The state of the `TDLG_CONTROLPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgControlpane {
    /// The default state.
    None,
}

impl Default for TdlgControlpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgControlpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgControlpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgControlpane {}

impl PartialOrd<i32> for TdlgControlpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgControlpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgControlpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgControlpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgControlpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_EXPANDEDCONTENT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgExpandedcontent {
    /// The default state.
    None,
}

impl Default for TdlgExpandedcontent {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgExpandedcontent {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgExpandedcontent {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgExpandedcontent {}

impl PartialOrd<i32> for TdlgExpandedcontent {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgExpandedcontent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgExpandedcontent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgExpandedcontent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgExpandedcontent {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_EXPANDEDFOOTERAREA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgExpandedfooterarea {
    /// The default state.
    None,
}

impl Default for TdlgExpandedfooterarea {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgExpandedfooterarea {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgExpandedfooterarea {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgExpandedfooterarea {}

impl PartialOrd<i32> for TdlgExpandedfooterarea {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgExpandedfooterarea {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgExpandedfooterarea {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgExpandedfooterarea {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgExpandedfooterarea {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_EXPANDOBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgExpandobutton {
    /// The default state.
    None,
    /// The `TDLGEBS_EXPANDEDHOVER` state.
    TdlgebsExpandedhover,
}

impl Default for TdlgExpandobutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgExpandobutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgExpandobutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgExpandobutton {}

impl PartialOrd<i32> for TdlgExpandobutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgExpandobutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgExpandobutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgExpandobutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgExpandobutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TdlgebsExpandedhover => windows_sys::Win32::UI::Controls::TDLGEBS_EXPANDEDHOVER,
        }
    }
}

/// The state of the `TDLG_EXPANDOTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgExpandotext {
    /// The default state.
    None,
}

impl Default for TdlgExpandotext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgExpandotext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgExpandotext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgExpandotext {}

impl PartialOrd<i32> for TdlgExpandotext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgExpandotext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgExpandotext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgExpandotext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgExpandotext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_FOOTNOTEAREA` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgFootnotearea {
    /// The default state.
    None,
}

impl Default for TdlgFootnotearea {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgFootnotearea {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgFootnotearea {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgFootnotearea {}

impl PartialOrd<i32> for TdlgFootnotearea {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgFootnotearea {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgFootnotearea {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgFootnotearea {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgFootnotearea {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_FOOTNOTEPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgFootnotepane {
    /// The default state.
    None,
}

impl Default for TdlgFootnotepane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgFootnotepane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgFootnotepane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgFootnotepane {}

impl PartialOrd<i32> for TdlgFootnotepane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgFootnotepane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgFootnotepane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgFootnotepane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgFootnotepane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_FOOTNOTESEPARATOR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgFootnoteseparator {
    /// The default state.
    None,
}

impl Default for TdlgFootnoteseparator {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgFootnoteseparator {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgFootnoteseparator {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgFootnoteseparator {}

impl PartialOrd<i32> for TdlgFootnoteseparator {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgFootnoteseparator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgFootnoteseparator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgFootnoteseparator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgFootnoteseparator {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_IMAGEALIGNMENT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgImagealignment {
    /// The default state.
    None,
}

impl Default for TdlgImagealignment {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgImagealignment {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgImagealignment {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgImagealignment {}

impl PartialOrd<i32> for TdlgImagealignment {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgImagealignment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgImagealignment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgImagealignment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgImagealignment {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_MAINICON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgMainicon {
    /// The default state.
    None,
}

impl Default for TdlgMainicon {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgMainicon {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgMainicon {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgMainicon {}

impl PartialOrd<i32> for TdlgMainicon {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgMainicon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgMainicon {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgMainicon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgMainicon {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_MAININSTRUCTIONPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgMaininstructionpane {
    /// The default state.
    None,
}

impl Default for TdlgMaininstructionpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgMaininstructionpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgMaininstructionpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgMaininstructionpane {}

impl PartialOrd<i32> for TdlgMaininstructionpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgMaininstructionpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgMaininstructionpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgMaininstructionpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgMaininstructionpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_PRIMARYPANEL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgPrimarypanel {
    /// The default state.
    None,
}

impl Default for TdlgPrimarypanel {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgPrimarypanel {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgPrimarypanel {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgPrimarypanel {}

impl PartialOrd<i32> for TdlgPrimarypanel {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgPrimarypanel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgPrimarypanel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgPrimarypanel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgPrimarypanel {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_PROGRESSBAR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgProgressbar {
    /// The default state.
    None,
}

impl Default for TdlgProgressbar {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgProgressbar {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgProgressbar {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgProgressbar {}

impl PartialOrd<i32> for TdlgProgressbar {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgProgressbar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgProgressbar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgProgressbar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgProgressbar {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_RADIOBUTTONPANE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgRadiobuttonpane {
    /// The default state.
    None,
}

impl Default for TdlgRadiobuttonpane {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgRadiobuttonpane {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgRadiobuttonpane {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgRadiobuttonpane {}

impl PartialOrd<i32> for TdlgRadiobuttonpane {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgRadiobuttonpane {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgRadiobuttonpane {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgRadiobuttonpane {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgRadiobuttonpane {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_SECONDARYPANEL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgSecondarypanel {
    /// The default state.
    None,
}

impl Default for TdlgSecondarypanel {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgSecondarypanel {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgSecondarypanel {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgSecondarypanel {}

impl PartialOrd<i32> for TdlgSecondarypanel {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgSecondarypanel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgSecondarypanel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgSecondarypanel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgSecondarypanel {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TDLG_VERIFICATIONTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TdlgVerificationtext {
    /// The default state.
    None,
}

impl Default for TdlgVerificationtext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TdlgVerificationtext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TdlgVerificationtext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TdlgVerificationtext {}

impl PartialOrd<i32> for TdlgVerificationtext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TdlgVerificationtext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TdlgVerificationtext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TdlgVerificationtext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TdlgVerificationtext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TEXT_BODYTITLE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextBodytitle {
    /// The default state.
    None,
}

impl Default for TextBodytitle {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextBodytitle {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextBodytitle {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextBodytitle {}

impl PartialOrd<i32> for TextBodytitle {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextBodytitle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextBodytitle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextBodytitle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextBodytitle {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TEXT_BODYTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextBodytext {
    /// The default state.
    None,
}

impl Default for TextBodytext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextBodytext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextBodytext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextBodytext {}

impl PartialOrd<i32> for TextBodytext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextBodytext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextBodytext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextBodytext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextBodytext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TEXT_CONTROLLABEL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextControllabel {
    /// The default state.
    None,
    /// The `TS_CONTROLLABEL_DISABLED` state.
    TsControllabelDisabled,
}

impl Default for TextControllabel {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextControllabel {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextControllabel {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextControllabel {}

impl PartialOrd<i32> for TextControllabel {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextControllabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextControllabel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextControllabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextControllabel {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsControllabelDisabled => windows_sys::Win32::UI::Controls::TS_CONTROLLABEL_DISABLED,
        }
    }
}

/// The state of the `TEXT_EXPANDED` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextExpanded {
    /// The default state.
    None,
}

impl Default for TextExpanded {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextExpanded {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextExpanded {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextExpanded {}

impl PartialOrd<i32> for TextExpanded {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextExpanded {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextExpanded {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextExpanded {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextExpanded {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TEXT_HYPERLINKTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextHyperlinktext {
    /// The default state.
    None,
    /// The `TS_HYPERLINK_DISABLED` state.
    TsHyperlinkDisabled,
}

impl Default for TextHyperlinktext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextHyperlinktext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextHyperlinktext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextHyperlinktext {}

impl PartialOrd<i32> for TextHyperlinktext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextHyperlinktext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextHyperlinktext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextHyperlinktext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextHyperlinktext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsHyperlinkDisabled => windows_sys::Win32::UI::Controls::TS_HYPERLINK_DISABLED,
        }
    }
}

/// The state of the `TEXT_INSTRUCTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextInstruction {
    /// The default state.
    None,
}

impl Default for TextInstruction {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextInstruction {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextInstruction {}

impl PartialOrd<i32> for TextInstruction {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextInstruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextInstruction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextInstruction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextInstruction {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TEXT_LABEL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextLabel {
    /// The default state.
    None,
}

impl Default for TextLabel {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextLabel {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextLabel {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextLabel {}

impl PartialOrd<i32> for TextLabel {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextLabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextLabel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextLabel {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TEXT_MAININSTRUCTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextMaininstruction {
    /// The default state.
    None,
}

impl Default for TextMaininstruction {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextMaininstruction {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextMaininstruction {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextMaininstruction {}

impl PartialOrd<i32> for TextMaininstruction {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextMaininstruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextMaininstruction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextMaininstruction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextMaininstruction {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TEXT_SECONDARYTEXT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TextSecondarytext {
    /// The default state.
    None,
}

impl Default for TextSecondarytext {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TextSecondarytext {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TextSecondarytext {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TextSecondarytext {}

impl PartialOrd<i32> for TextSecondarytext {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TextSecondarytext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TextSecondarytext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TextSecondarytext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TextSecondarytext {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TP_BUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TpButton {
    /// The default state.
    None,
    /// The `TS_CHECKED` state.
    TsChecked,
}

impl Default for TpButton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TpButton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TpButton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TpButton {}

impl PartialOrd<i32> for TpButton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TpButton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TpButton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TpButton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TpButton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsChecked => windows_sys::Win32::UI::Controls::TS_CHECKED,
        }
    }
}

/// The state of the `TP_DROPDOWNBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TpDropdownbutton {
    /// The default state.
    None,
    /// The `TS_CHECKED` state.
    TsChecked,
}

impl Default for TpDropdownbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TpDropdownbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TpDropdownbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TpDropdownbutton {}

impl PartialOrd<i32> for TpDropdownbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TpDropdownbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TpDropdownbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TpDropdownbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TpDropdownbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsChecked => windows_sys::Win32::UI::Controls::TS_CHECKED,
        }
    }
}

/// The state of the `TP_DROPDOWNBUTTONGLYPH` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TpDropdownbuttonglyph {
    /// The default state.
    None,
    /// The `TS_CHECKED` state.
    TsChecked,
}

impl Default for TpDropdownbuttonglyph {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TpDropdownbuttonglyph {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TpDropdownbuttonglyph {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TpDropdownbuttonglyph {}

impl PartialOrd<i32> for TpDropdownbuttonglyph {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TpDropdownbuttonglyph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TpDropdownbuttonglyph {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TpDropdownbuttonglyph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TpDropdownbuttonglyph {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsChecked => windows_sys::Win32::UI::Controls::TS_CHECKED,
        }
    }
}

/// The state of the `TP_SEPARATOR` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TpSeparator {
    /// The default state.
    None,
    /// The `TS_CHECKED` state.
    TsChecked,
}

impl Default for TpSeparator {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TpSeparator {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TpSeparator {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TpSeparator {}

impl PartialOrd<i32> for TpSeparator {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TpSeparator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TpSeparator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TpSeparator {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TpSeparator {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsChecked => windows_sys::Win32::UI::Controls::TS_CHECKED,
        }
    }
}

/// The state of the `TP_SEPARATORVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TpSeparatorvert {
    /// The default state.
    None,
    /// The `TS_CHECKED` state.
    TsChecked,
}

impl Default for TpSeparatorvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TpSeparatorvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TpSeparatorvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TpSeparatorvert {}

impl PartialOrd<i32> for TpSeparatorvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TpSeparatorvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TpSeparatorvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TpSeparatorvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TpSeparatorvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsChecked => windows_sys::Win32::UI::Controls::TS_CHECKED,
        }
    }
}

/// The state of the `TP_SPLITBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TpSplitbutton {
    /// The default state.
    None,
    /// The `TS_CHECKED` state.
    TsChecked,
}

impl Default for TpSplitbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TpSplitbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TpSplitbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TpSplitbutton {}

impl PartialOrd<i32> for TpSplitbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TpSplitbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TpSplitbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TpSplitbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TpSplitbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsChecked => windows_sys::Win32::UI::Controls::TS_CHECKED,
        }
    }
}

/// The state of the `TP_SPLITBUTTONDROPDOWN` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TpSplitbuttondropdown {
    /// The default state.
    None,
    /// The `TS_CHECKED` state.
    TsChecked,
}

impl Default for TpSplitbuttondropdown {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TpSplitbuttondropdown {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TpSplitbuttondropdown {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TpSplitbuttondropdown {}

impl PartialOrd<i32> for TpSplitbuttondropdown {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TpSplitbuttondropdown {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TpSplitbuttondropdown {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TpSplitbuttondropdown {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TpSplitbuttondropdown {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsChecked => windows_sys::Win32::UI::Controls::TS_CHECKED,
        }
    }
}

/// The state of the `TTP_BALLOON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TtpBalloon {
    /// The default state.
    None,
    /// The `TTBS_LINK` state.
    TtbsLink,
}

impl Default for TtpBalloon {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TtpBalloon {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TtpBalloon {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TtpBalloon {}

impl PartialOrd<i32> for TtpBalloon {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TtpBalloon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TtpBalloon {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TtpBalloon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TtpBalloon {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtbsLink => windows_sys::Win32::UI::Controls::TTBS_LINK,
        }
    }
}

/// The state of the `TTP_BALLOONSTEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TtpBalloonstem {
    /// The default state.
    None,
    /// The `TTBSS_POINTINGUPLEFTWALL` state.
    TtbssPointingupleftwall,
}

impl Default for TtpBalloonstem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TtpBalloonstem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TtpBalloonstem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TtpBalloonstem {}

impl PartialOrd<i32> for TtpBalloonstem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TtpBalloonstem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TtpBalloonstem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TtpBalloonstem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TtpBalloonstem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtbssPointingupleftwall => windows_sys::Win32::UI::Controls::TTBSS_POINTINGUPLEFTWALL,
        }
    }
}

/// The state of the `TTP_BALLOONTITLE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TtpBalloontitle {
    /// The default state.
    None,
}

impl Default for TtpBalloontitle {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TtpBalloontitle {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TtpBalloontitle {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TtpBalloontitle {}

impl PartialOrd<i32> for TtpBalloontitle {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TtpBalloontitle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TtpBalloontitle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TtpBalloontitle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TtpBalloontitle {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TTP_CLOSE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TtpClose {
    /// The default state.
    None,
    /// The `TTCS_HOT` state.
    TtcsHot,
}

impl Default for TtpClose {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TtpClose {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TtpClose {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TtpClose {}

impl PartialOrd<i32> for TtpClose {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TtpClose {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TtpClose {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TtpClose {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TtpClose {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtcsHot => windows_sys::Win32::UI::Controls::TTCS_HOT,
        }
    }
}

/// The state of the `TTP_STANDARD` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TtpStandard {
    /// The default state.
    None,
    /// The `TTSS_LINK` state.
    TtssLink,
}

impl Default for TtpStandard {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TtpStandard {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TtpStandard {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TtpStandard {}

impl PartialOrd<i32> for TtpStandard {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TtpStandard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TtpStandard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TtpStandard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TtpStandard {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtssLink => windows_sys::Win32::UI::Controls::TTSS_LINK,
        }
    }
}

/// The state of the `TTP_STANDARDTITLE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TtpStandardtitle {
    /// The default state.
    None,
    /// The `TTSS_LINK` state.
    TtssLink,
}

impl Default for TtpStandardtitle {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TtpStandardtitle {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TtpStandardtitle {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TtpStandardtitle {}

impl PartialOrd<i32> for TtpStandardtitle {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TtpStandardtitle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TtpStandardtitle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TtpStandardtitle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TtpStandardtitle {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtssLink => windows_sys::Win32::UI::Controls::TTSS_LINK,
        }
    }
}

/// The state of the `TTP_WRENCH` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TtpWrench {
    /// The default state.
    None,
    /// The `TTWS_NORMAL` state.
    TtwsNormal,
}

impl Default for TtpWrench {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TtpWrench {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TtpWrench {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TtpWrench {}

impl PartialOrd<i32> for TtpWrench {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TtpWrench {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TtpWrench {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TtpWrench {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TtpWrench {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TtwsNormal => windows_sys::Win32::UI::Controls::TTWS_NORMAL,
        }
    }
}

/// The state of the `TKP_THUMB` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpThumb {
    /// The default state.
    None,
    /// The `TUS_DISABLED` state.
    TusDisabled,
}

impl Default for TkpThumb {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpThumb {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpThumb {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpThumb {}

impl PartialOrd<i32> for TkpThumb {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpThumb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpThumb {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpThumb {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpThumb {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TusDisabled => windows_sys::Win32::UI::Controls::TUS_DISABLED,
        }
    }
}

/// The state of the `TKP_THUMBBOTTOM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpThumbbottom {
    /// The default state.
    None,
    /// The `TUBS_DISABLED` state.
    TubsDisabled,
}

impl Default for TkpThumbbottom {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpThumbbottom {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpThumbbottom {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpThumbbottom {}

impl PartialOrd<i32> for TkpThumbbottom {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpThumbbottom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpThumbbottom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpThumbbottom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpThumbbottom {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TubsDisabled => windows_sys::Win32::UI::Controls::TUBS_DISABLED,
        }
    }
}

/// The state of the `TKP_THUMBLEFT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpThumbleft {
    /// The default state.
    None,
    /// The `TUVLS_DISABLED` state.
    TuvlsDisabled,
}

impl Default for TkpThumbleft {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpThumbleft {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpThumbleft {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpThumbleft {}

impl PartialOrd<i32> for TkpThumbleft {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpThumbleft {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpThumbleft {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpThumbleft {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpThumbleft {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TuvlsDisabled => windows_sys::Win32::UI::Controls::TUVLS_DISABLED,
        }
    }
}

/// The state of the `TKP_THUMBRIGHT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpThumbright {
    /// The default state.
    None,
    /// The `TUVRS_DISABLED` state.
    TuvrsDisabled,
}

impl Default for TkpThumbright {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpThumbright {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpThumbright {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpThumbright {}

impl PartialOrd<i32> for TkpThumbright {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpThumbright {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpThumbright {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpThumbright {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpThumbright {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TuvrsDisabled => windows_sys::Win32::UI::Controls::TUVRS_DISABLED,
        }
    }
}

/// The state of the `TKP_THUMBTOP` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpThumbtop {
    /// The default state.
    None,
    /// The `TUTS_DISABLED` state.
    TutsDisabled,
}

impl Default for TkpThumbtop {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpThumbtop {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpThumbtop {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpThumbtop {}

impl PartialOrd<i32> for TkpThumbtop {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpThumbtop {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpThumbtop {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpThumbtop {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpThumbtop {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TutsDisabled => windows_sys::Win32::UI::Controls::TUTS_DISABLED,
        }
    }
}

/// The state of the `TKP_THUMBVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpThumbvert {
    /// The default state.
    None,
    /// The `TUVS_DISABLED` state.
    TuvsDisabled,
}

impl Default for TkpThumbvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpThumbvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpThumbvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpThumbvert {}

impl PartialOrd<i32> for TkpThumbvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpThumbvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpThumbvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpThumbvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpThumbvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TuvsDisabled => windows_sys::Win32::UI::Controls::TUVS_DISABLED,
        }
    }
}

/// The state of the `TKP_TICS` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpTics {
    /// The default state.
    None,
    /// The `TSS_NORMAL` state.
    TssNormal,
}

impl Default for TkpTics {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpTics {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpTics {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpTics {}

impl PartialOrd<i32> for TkpTics {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpTics {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpTics {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpTics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpTics {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TssNormal => windows_sys::Win32::UI::Controls::TSS_NORMAL,
        }
    }
}

/// The state of the `TKP_TICSVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpTicsvert {
    /// The default state.
    None,
    /// The `TSVS_NORMAL` state.
    TsvsNormal,
}

impl Default for TkpTicsvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpTicsvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpTicsvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpTicsvert {}

impl PartialOrd<i32> for TkpTicsvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpTicsvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpTicsvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpTicsvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpTicsvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TsvsNormal => windows_sys::Win32::UI::Controls::TSVS_NORMAL,
        }
    }
}

/// The state of the `TKP_TRACK` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpTrack {
    /// The default state.
    None,
    /// The `TRS_NORMAL` state.
    TrsNormal,
}

impl Default for TkpTrack {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpTrack {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpTrack {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpTrack {}

impl PartialOrd<i32> for TkpTrack {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpTrack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpTrack {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpTrack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpTrack {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TrsNormal => windows_sys::Win32::UI::Controls::TRS_NORMAL,
        }
    }
}

/// The state of the `TKP_TRACKVERT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TkpTrackvert {
    /// The default state.
    None,
    /// The `TRVS_NORMAL` state.
    TrvsNormal,
}

impl Default for TkpTrackvert {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TkpTrackvert {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TkpTrackvert {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TkpTrackvert {}

impl PartialOrd<i32> for TkpTrackvert {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TkpTrackvert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TkpTrackvert {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TkpTrackvert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TkpTrackvert {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TrvsNormal => windows_sys::Win32::UI::Controls::TRVS_NORMAL,
        }
    }
}

/// The state of the `TNP_ANIMBACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TnpAnimbackground {
    /// The default state.
    None,
}

impl Default for TnpAnimbackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TnpAnimbackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TnpAnimbackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TnpAnimbackground {}

impl PartialOrd<i32> for TnpAnimbackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TnpAnimbackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TnpAnimbackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TnpAnimbackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TnpAnimbackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TNP_BACKGROUND` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TnpBackground {
    /// The default state.
    None,
}

impl Default for TnpBackground {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TnpBackground {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TnpBackground {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TnpBackground {}

impl PartialOrd<i32> for TnpBackground {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TnpBackground {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TnpBackground {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TnpBackground {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TnpBackground {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TVP_BRANCH` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TvpBranch {
    /// The default state.
    None,
}

impl Default for TvpBranch {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TvpBranch {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TvpBranch {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TvpBranch {}

impl PartialOrd<i32> for TvpBranch {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TvpBranch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TvpBranch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TvpBranch {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TvpBranch {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `TVP_GLYPH` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TvpGlyph {
    /// The default state.
    None,
    /// The `GLPS_CLOSED` state.
    GlpsClosed,
}

impl Default for TvpGlyph {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TvpGlyph {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TvpGlyph {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TvpGlyph {}

impl PartialOrd<i32> for TvpGlyph {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TvpGlyph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TvpGlyph {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TvpGlyph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TvpGlyph {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::GlpsClosed => windows_sys::Win32::UI::Controls::GLPS_CLOSED,
        }
    }
}

/// The state of the `TVP_HOTGLYPH` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TvpHotglyph {
    /// The default state.
    None,
    /// The `HGLPS_CLOSED` state.
    HglpsClosed,
}

impl Default for TvpHotglyph {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TvpHotglyph {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TvpHotglyph {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TvpHotglyph {}

impl PartialOrd<i32> for TvpHotglyph {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TvpHotglyph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TvpHotglyph {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TvpHotglyph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TvpHotglyph {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HglpsClosed => windows_sys::Win32::UI::Controls::HGLPS_CLOSED,
        }
    }
}

/// The state of the `TVP_TREEITEM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum TvpTreeitem {
    /// The default state.
    None,
    /// The `TREIS_DISABLED` state.
    TreisDisabled,
}

impl Default for TvpTreeitem {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for TvpTreeitem {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for TvpTreeitem {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for TvpTreeitem {}

impl PartialOrd<i32> for TvpTreeitem {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for TvpTreeitem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for TvpTreeitem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for TvpTreeitem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl TvpTreeitem {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::TreisDisabled => windows_sys::Win32::UI::Controls::TREIS_DISABLED,
        }
    }
}

/// The state of the `WP_CAPTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpCaption {
    /// The default state.
    None,
    /// The `CS_ACTIVE` state.
    CsActive,
}

impl Default for WpCaption {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpCaption {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpCaption {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpCaption {}

impl PartialOrd<i32> for WpCaption {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpCaption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpCaption {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpCaption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpCaption {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CsActive => windows_sys::Win32::UI::Controls::CS_ACTIVE,
        }
    }
}

/// The state of the `WP_CAPTIONSIZINGTEMPLATE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpCaptionsizingtemplate {
    /// The default state.
    None,
}

impl Default for WpCaptionsizingtemplate {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpCaptionsizingtemplate {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpCaptionsizingtemplate {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpCaptionsizingtemplate {}

impl PartialOrd<i32> for WpCaptionsizingtemplate {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpCaptionsizingtemplate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpCaptionsizingtemplate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpCaptionsizingtemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpCaptionsizingtemplate {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_CLOSEBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpClosebutton {
    /// The default state.
    None,
    /// The `CBS_DISABLED` state.
    CbsDisabled,
}

impl Default for WpClosebutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpClosebutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpClosebutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpClosebutton {}

impl PartialOrd<i32> for WpClosebutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpClosebutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpClosebutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpClosebutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpClosebutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::CbsDisabled => windows_sys::Win32::UI::Controls::CBS_DISABLED,
        }
    }
}

/// The state of the `WP_DIALOG` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpDialog {
    /// The default state.
    None,
}

impl Default for WpDialog {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpDialog {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpDialog {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpDialog {}

impl PartialOrd<i32> for WpDialog {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpDialog {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpDialog {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpDialog {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpDialog {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_FRAME` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpFrame {
    /// The default state.
    None,
    /// The `FS_ACTIVE` state.
    FsActive,
}

impl Default for WpFrame {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpFrame {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpFrame {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpFrame {}

impl PartialOrd<i32> for WpFrame {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpFrame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpFrame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpFrame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpFrame {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::FsActive => windows_sys::Win32::UI::Controls::FS_ACTIVE,
        }
    }
}

/// The state of the `WP_FRAMEBOTTOM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpFramebottom {
    /// The default state.
    None,
}

impl Default for WpFramebottom {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpFramebottom {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpFramebottom {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpFramebottom {}

impl PartialOrd<i32> for WpFramebottom {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpFramebottom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpFramebottom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpFramebottom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpFramebottom {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_FRAMEBOTTOMSIZINGTEMPLATE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpFramebottomsizingtemplate {
    /// The default state.
    None,
}

impl Default for WpFramebottomsizingtemplate {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpFramebottomsizingtemplate {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpFramebottomsizingtemplate {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpFramebottomsizingtemplate {}

impl PartialOrd<i32> for WpFramebottomsizingtemplate {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpFramebottomsizingtemplate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpFramebottomsizingtemplate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpFramebottomsizingtemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpFramebottomsizingtemplate {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_FRAMELEFT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpFrameleft {
    /// The default state.
    None,
}

impl Default for WpFrameleft {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpFrameleft {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpFrameleft {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpFrameleft {}

impl PartialOrd<i32> for WpFrameleft {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpFrameleft {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpFrameleft {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpFrameleft {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpFrameleft {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_FRAMELEFTSIZINGTEMPLATE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpFrameleftsizingtemplate {
    /// The default state.
    None,
}

impl Default for WpFrameleftsizingtemplate {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpFrameleftsizingtemplate {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpFrameleftsizingtemplate {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpFrameleftsizingtemplate {}

impl PartialOrd<i32> for WpFrameleftsizingtemplate {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpFrameleftsizingtemplate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpFrameleftsizingtemplate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpFrameleftsizingtemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpFrameleftsizingtemplate {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_FRAMERIGHT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpFrameright {
    /// The default state.
    None,
}

impl Default for WpFrameright {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpFrameright {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpFrameright {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpFrameright {}

impl PartialOrd<i32> for WpFrameright {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpFrameright {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpFrameright {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpFrameright {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpFrameright {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_FRAMERIGHTSIZINGTEMPLATE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpFramerightsizingtemplate {
    /// The default state.
    None,
}

impl Default for WpFramerightsizingtemplate {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpFramerightsizingtemplate {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpFramerightsizingtemplate {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpFramerightsizingtemplate {}

impl PartialOrd<i32> for WpFramerightsizingtemplate {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpFramerightsizingtemplate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpFramerightsizingtemplate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpFramerightsizingtemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpFramerightsizingtemplate {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_HELPBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpHelpbutton {
    /// The default state.
    None,
    /// The `HBS_DISABLED` state.
    HbsDisabled,
}

impl Default for WpHelpbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpHelpbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpHelpbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpHelpbutton {}

impl PartialOrd<i32> for WpHelpbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpHelpbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpHelpbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpHelpbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpHelpbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HbsDisabled => windows_sys::Win32::UI::Controls::HBS_DISABLED,
        }
    }
}

/// The state of the `WP_HORZSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpHorzscroll {
    /// The default state.
    None,
    /// The `HSS_DISABLED` state.
    HssDisabled,
}

impl Default for WpHorzscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpHorzscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpHorzscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpHorzscroll {}

impl PartialOrd<i32> for WpHorzscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpHorzscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpHorzscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpHorzscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpHorzscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HssDisabled => windows_sys::Win32::UI::Controls::HSS_DISABLED,
        }
    }
}

/// The state of the `WP_HORZTHUMB` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpHorzthumb {
    /// The default state.
    None,
    /// The `HTS_DISABLED` state.
    HtsDisabled,
}

impl Default for WpHorzthumb {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpHorzthumb {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpHorzthumb {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpHorzthumb {}

impl PartialOrd<i32> for WpHorzthumb {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpHorzthumb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpHorzthumb {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpHorzthumb {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpHorzthumb {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::HtsDisabled => windows_sys::Win32::UI::Controls::HTS_DISABLED,
        }
    }
}

/// The state of the `WP_MAXBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMaxbutton {
    /// The default state.
    None,
    /// The `MAXBS_DISABLED` state.
    MaxbsDisabled,
}

impl Default for WpMaxbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMaxbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMaxbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMaxbutton {}

impl PartialOrd<i32> for WpMaxbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMaxbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMaxbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMaxbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMaxbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MaxbsDisabled => windows_sys::Win32::UI::Controls::MAXBS_DISABLED,
        }
    }
}

/// The state of the `WP_MAXCAPTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMaxcaption {
    /// The default state.
    None,
    /// The `MXCS_ACTIVE` state.
    MxcsActive,
}

impl Default for WpMaxcaption {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMaxcaption {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMaxcaption {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMaxcaption {}

impl PartialOrd<i32> for WpMaxcaption {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMaxcaption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMaxcaption {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMaxcaption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMaxcaption {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MxcsActive => windows_sys::Win32::UI::Controls::MXCS_ACTIVE,
        }
    }
}

/// The state of the `WP_MDICLOSEBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMdiclosebutton {
    /// The default state.
    None,
}

impl Default for WpMdiclosebutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMdiclosebutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMdiclosebutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMdiclosebutton {}

impl PartialOrd<i32> for WpMdiclosebutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMdiclosebutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMdiclosebutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMdiclosebutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMdiclosebutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_MDIHELPBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMdihelpbutton {
    /// The default state.
    None,
}

impl Default for WpMdihelpbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMdihelpbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMdihelpbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMdihelpbutton {}

impl PartialOrd<i32> for WpMdihelpbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMdihelpbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMdihelpbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMdihelpbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMdihelpbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_MDIMINBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMdiminbutton {
    /// The default state.
    None,
}

impl Default for WpMdiminbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMdiminbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMdiminbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMdiminbutton {}

impl PartialOrd<i32> for WpMdiminbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMdiminbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMdiminbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMdiminbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMdiminbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_MDIRESTOREBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMdirestorebutton {
    /// The default state.
    None,
}

impl Default for WpMdirestorebutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMdirestorebutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMdirestorebutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMdirestorebutton {}

impl PartialOrd<i32> for WpMdirestorebutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMdirestorebutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMdirestorebutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMdirestorebutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMdirestorebutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_MDISYSBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMdisysbutton {
    /// The default state.
    None,
}

impl Default for WpMdisysbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMdisysbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMdisysbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMdisysbutton {}

impl PartialOrd<i32> for WpMdisysbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMdisysbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMdisysbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMdisysbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMdisysbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_MINBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMinbutton {
    /// The default state.
    None,
    /// The `MINBS_DISABLED` state.
    MinbsDisabled,
}

impl Default for WpMinbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMinbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMinbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMinbutton {}

impl PartialOrd<i32> for WpMinbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMinbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMinbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMinbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMinbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MinbsDisabled => windows_sys::Win32::UI::Controls::MINBS_DISABLED,
        }
    }
}

/// The state of the `WP_MINCAPTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpMincaption {
    /// The default state.
    None,
    /// The `MNCS_ACTIVE` state.
    MncsActive,
}

impl Default for WpMincaption {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpMincaption {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpMincaption {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpMincaption {}

impl PartialOrd<i32> for WpMincaption {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpMincaption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpMincaption {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpMincaption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpMincaption {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::MncsActive => windows_sys::Win32::UI::Controls::MNCS_ACTIVE,
        }
    }
}

/// The state of the `WP_RESTOREBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpRestorebutton {
    /// The default state.
    None,
    /// The `RBS_DISABLED` state.
    RbsDisabled,
}

impl Default for WpRestorebutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpRestorebutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpRestorebutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpRestorebutton {}

impl PartialOrd<i32> for WpRestorebutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpRestorebutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpRestorebutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpRestorebutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpRestorebutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::RbsDisabled => windows_sys::Win32::UI::Controls::RBS_DISABLED,
        }
    }
}

/// The state of the `WP_SMALLCAPTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallcaption {
    /// The default state.
    None,
}

impl Default for WpSmallcaption {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallcaption {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallcaption {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallcaption {}

impl PartialOrd<i32> for WpSmallcaption {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallcaption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallcaption {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallcaption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallcaption {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLCAPTIONSIZINGTEMPLATE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallcaptionsizingtemplate {
    /// The default state.
    None,
}

impl Default for WpSmallcaptionsizingtemplate {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallcaptionsizingtemplate {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallcaptionsizingtemplate {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallcaptionsizingtemplate {}

impl PartialOrd<i32> for WpSmallcaptionsizingtemplate {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallcaptionsizingtemplate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallcaptionsizingtemplate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallcaptionsizingtemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallcaptionsizingtemplate {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLCLOSEBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallclosebutton {
    /// The default state.
    None,
}

impl Default for WpSmallclosebutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallclosebutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallclosebutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallclosebutton {}

impl PartialOrd<i32> for WpSmallclosebutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallclosebutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallclosebutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallclosebutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallclosebutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLFRAMEBOTTOM` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallframebottom {
    /// The default state.
    None,
}

impl Default for WpSmallframebottom {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallframebottom {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallframebottom {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallframebottom {}

impl PartialOrd<i32> for WpSmallframebottom {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallframebottom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallframebottom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallframebottom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallframebottom {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLFRAMEBOTTOMSIZINGTEMPLATE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallframebottomsizingtemplate {
    /// The default state.
    None,
}

impl Default for WpSmallframebottomsizingtemplate {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallframebottomsizingtemplate {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallframebottomsizingtemplate {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallframebottomsizingtemplate {}

impl PartialOrd<i32> for WpSmallframebottomsizingtemplate {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallframebottomsizingtemplate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallframebottomsizingtemplate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallframebottomsizingtemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallframebottomsizingtemplate {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLFRAMELEFT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallframeleft {
    /// The default state.
    None,
}

impl Default for WpSmallframeleft {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallframeleft {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallframeleft {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallframeleft {}

impl PartialOrd<i32> for WpSmallframeleft {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallframeleft {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallframeleft {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallframeleft {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallframeleft {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLFRAMELEFTSIZINGTEMPLATE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallframeleftsizingtemplate {
    /// The default state.
    None,
}

impl Default for WpSmallframeleftsizingtemplate {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallframeleftsizingtemplate {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallframeleftsizingtemplate {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallframeleftsizingtemplate {}

impl PartialOrd<i32> for WpSmallframeleftsizingtemplate {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallframeleftsizingtemplate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallframeleftsizingtemplate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallframeleftsizingtemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallframeleftsizingtemplate {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLFRAMERIGHT` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallframeright {
    /// The default state.
    None,
}

impl Default for WpSmallframeright {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallframeright {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallframeright {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallframeright {}

impl PartialOrd<i32> for WpSmallframeright {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallframeright {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallframeright {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallframeright {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallframeright {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLFRAMERIGHTSIZINGTEMPLATE` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallframerightsizingtemplate {
    /// The default state.
    None,
}

impl Default for WpSmallframerightsizingtemplate {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallframerightsizingtemplate {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallframerightsizingtemplate {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallframerightsizingtemplate {}

impl PartialOrd<i32> for WpSmallframerightsizingtemplate {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallframerightsizingtemplate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallframerightsizingtemplate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallframerightsizingtemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallframerightsizingtemplate {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLMAXCAPTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallmaxcaption {
    /// The default state.
    None,
}

impl Default for WpSmallmaxcaption {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallmaxcaption {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallmaxcaption {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallmaxcaption {}

impl PartialOrd<i32> for WpSmallmaxcaption {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallmaxcaption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallmaxcaption {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallmaxcaption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallmaxcaption {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SMALLMINCAPTION` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSmallmincaption {
    /// The default state.
    None,
}

impl Default for WpSmallmincaption {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSmallmincaption {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSmallmincaption {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSmallmincaption {}

impl PartialOrd<i32> for WpSmallmincaption {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSmallmincaption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSmallmincaption {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSmallmincaption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSmallmincaption {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
        }
    }
}

/// The state of the `WP_SYSBUTTON` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpSysbutton {
    /// The default state.
    None,
    /// The `SBS_DISABLED` state.
    SbsDisabled,
}

impl Default for WpSysbutton {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpSysbutton {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpSysbutton {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpSysbutton {}

impl PartialOrd<i32> for WpSysbutton {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpSysbutton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpSysbutton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpSysbutton {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpSysbutton {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::SbsDisabled => windows_sys::Win32::UI::Controls::SBS_DISABLED,
        }
    }
}

/// The state of the `WP_VERTSCROLL` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpVertscroll {
    /// The default state.
    None,
    /// The `VSS_DISABLED` state.
    VssDisabled,
}

impl Default for WpVertscroll {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpVertscroll {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpVertscroll {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpVertscroll {}

impl PartialOrd<i32> for WpVertscroll {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpVertscroll {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpVertscroll {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpVertscroll {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpVertscroll {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::VssDisabled => windows_sys::Win32::UI::Controls::VSS_DISABLED,
        }
    }
}

/// The state of the `WP_VERTTHUMB` part.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum WpVertthumb {
    /// The default state.
    None,
    /// The `VTS_DISABLED` state.
    VtsDisabled,
}

impl Default for WpVertthumb {
    fn default() -> Self {
        Self::None
    }
}

impl PartialEq<i32> for WpVertthumb {
    fn eq(&self, other: &i32) -> bool {
        self.state() == *other
    }
}

impl PartialEq for WpVertthumb {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl Eq for WpVertthumb {}

impl PartialOrd<i32> for WpVertthumb {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.state().partial_cmp(other)
    }
}

impl PartialOrd for WpVertthumb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.state().partial_cmp(&other.state())
    }
}

impl Ord for WpVertthumb {
    fn cmp(&self, other: &Self) -> Ordering {
        self.state().cmp(&other.state())
    }
}

impl Hash for WpVertthumb {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state().hash(state);
    }
}

impl WpVertthumb {
    /// Returns the state of this part.
    fn state(self) -> i32 {
        match self {
            Self::None => 0,
            Self::VtsDisabled => windows_sys::Win32::UI::Controls::VTS_DISABLED,
        }
    }
}

impl Part {
    /// Returns the part and state for this part.
    pub(super) fn part_and_state(self) -> (i32, i32) {
        match self {
            Self::BpCheckbox(state) => (windows_sys::Win32::UI::Controls::BP_CHECKBOX, state.state()),
            Self::BpCommandlink(state) => (windows_sys::Win32::UI::Controls::BP_COMMANDLINK, state.state()),
            Self::BpCommandlinkglyph(state) => (windows_sys::Win32::UI::Controls::BP_COMMANDLINKGLYPH, state.state()),
            Self::BpGroupbox(state) => (windows_sys::Win32::UI::Controls::BP_GROUPBOX, state.state()),
            Self::BpPushbutton(state) => (windows_sys::Win32::UI::Controls::BP_PUSHBUTTON, state.state()),
            Self::BpRadiobutton(state) => (windows_sys::Win32::UI::Controls::BP_RADIOBUTTON, state.state()),
            Self::BpUserbutton(state) => (windows_sys::Win32::UI::Controls::BP_USERBUTTON, state.state()),
            Self::ClpTime(state) => (windows_sys::Win32::UI::Controls::CLP_TIME, state.state()),
            Self::CpBackground(state) => (windows_sys::Win32::UI::Controls::CP_BACKGROUND, state.state()),
            Self::CpBorder(state) => (windows_sys::Win32::UI::Controls::CP_BORDER, state.state()),
            Self::CpCuebanner(state) => (windows_sys::Win32::UI::Controls::CP_CUEBANNER, state.state()),
            Self::CpDropdownbutton(state) => (windows_sys::Win32::UI::Controls::CP_DROPDOWNBUTTON, state.state()),
            Self::CpDropdownbuttonleft(state) => (windows_sys::Win32::UI::Controls::CP_DROPDOWNBUTTONLEFT, state.state()),
            Self::CpDropdownbuttonright(state) => (windows_sys::Win32::UI::Controls::CP_DROPDOWNBUTTONRIGHT, state.state()),
            Self::CpTransparentbackground(state) => (windows_sys::Win32::UI::Controls::CP_TRANSPARENTBACKGROUND, state.state()),
            Self::CpReadonly(state) => (windows_sys::Win32::UI::Controls::CP_READONLY, state.state()),
            Self::CsstTab(state) => (windows_sys::Win32::UI::Controls::CSST_TAB, state.state()),
            Self::CpanelBannerarea(state) => (windows_sys::Win32::UI::Controls::CPANEL_BANNERAREA, state.state()),
            Self::CpanelBodytext(state) => (windows_sys::Win32::UI::Controls::CPANEL_BODYTEXT, state.state()),
            Self::CpanelBodytitle(state) => (windows_sys::Win32::UI::Controls::CPANEL_BODYTITLE, state.state()),
            Self::CpanelButton(state) => (windows_sys::Win32::UI::Controls::CPANEL_BUTTON, state.state()),
            Self::CpanelContentlink(state) => (windows_sys::Win32::UI::Controls::CPANEL_CONTENTLINK, state.state()),
            Self::CpanelContentpane(state) => (windows_sys::Win32::UI::Controls::CPANEL_CONTENTPANE, state.state()),
            Self::CpanelContentpanelabel(state) => (windows_sys::Win32::UI::Controls::CPANEL_CONTENTPANELABEL, state.state()),
            Self::CpanelContentpaneline(state) => (windows_sys::Win32::UI::Controls::CPANEL_CONTENTPANELINE, state.state()),
            Self::CpanelGrouptext(state) => (windows_sys::Win32::UI::Controls::CPANEL_GROUPTEXT, state.state()),
            Self::CpanelHelplink(state) => (windows_sys::Win32::UI::Controls::CPANEL_HELPLINK, state.state()),
            Self::CpanelLargecommandarea(state) => (windows_sys::Win32::UI::Controls::CPANEL_LARGECOMMANDAREA, state.state()),
            Self::CpanelMessagetext(state) => (windows_sys::Win32::UI::Controls::CPANEL_MESSAGETEXT, state.state()),
            Self::CpanelNavigationpane(state) => (windows_sys::Win32::UI::Controls::CPANEL_NAVIGATIONPANE, state.state()),
            Self::CpanelNavigationpanelabel(state) => (windows_sys::Win32::UI::Controls::CPANEL_NAVIGATIONPANELABEL, state.state()),
            Self::CpanelNavigationpaneline(state) => (windows_sys::Win32::UI::Controls::CPANEL_NAVIGATIONPANELINE, state.state()),
            Self::CpanelSectiontitlelink(state) => (windows_sys::Win32::UI::Controls::CPANEL_SECTIONTITLELINK, state.state()),
            Self::CpanelSmallcommandarea(state) => (windows_sys::Win32::UI::Controls::CPANEL_SMALLCOMMANDAREA, state.state()),
            Self::CpanelTasklink(state) => (windows_sys::Win32::UI::Controls::CPANEL_TASKLINK, state.state()),
            Self::CpanelTitle(state) => (windows_sys::Win32::UI::Controls::CPANEL_TITLE, state.state()),
            Self::DpDateborder(state) => (windows_sys::Win32::UI::Controls::DP_DATEBORDER, state.state()),
            Self::DpDatetext(state) => (windows_sys::Win32::UI::Controls::DP_DATETEXT, state.state()),
            Self::DpShowcalendarbuttonright(state) => (windows_sys::Win32::UI::Controls::DP_SHOWCALENDARBUTTONRIGHT, state.state()),
            Self::DdCopy(state) => (windows_sys::Win32::UI::Controls::DD_COPY, state.state()),
            Self::DdCreatelink(state) => (windows_sys::Win32::UI::Controls::DD_CREATELINK, state.state()),
            Self::DdImagebg(state) => (windows_sys::Win32::UI::Controls::DD_IMAGEBG, state.state()),
            Self::DdMove(state) => (windows_sys::Win32::UI::Controls::DD_MOVE, state.state()),
            Self::DdNone(state) => (windows_sys::Win32::UI::Controls::DD_NONE, state.state()),
            Self::DdTextbg(state) => (windows_sys::Win32::UI::Controls::DD_TEXTBG, state.state()),
            Self::DdUpdatemetadata(state) => (windows_sys::Win32::UI::Controls::DD_UPDATEMETADATA, state.state()),
            Self::DdWarning(state) => (windows_sys::Win32::UI::Controls::DD_WARNING, state.state()),
            Self::EpBackground(state) => (windows_sys::Win32::UI::Controls::EP_BACKGROUND, state.state()),
            Self::EpBackgroundwithborder(state) => (windows_sys::Win32::UI::Controls::EP_BACKGROUNDWITHBORDER, state.state()),
            Self::EpCaret(state) => (windows_sys::Win32::UI::Controls::EP_CARET, state.state()),
            Self::EpEditborderHscroll(state) => (windows_sys::Win32::UI::Controls::EP_EDITBORDER_HSCROLL, state.state()),
            Self::EpEditborderHvscroll(state) => (windows_sys::Win32::UI::Controls::EP_EDITBORDER_HVSCROLL, state.state()),
            Self::EpEditborderNoscroll(state) => (windows_sys::Win32::UI::Controls::EP_EDITBORDER_NOSCROLL, state.state()),
            Self::EpEditborderVscroll(state) => (windows_sys::Win32::UI::Controls::EP_EDITBORDER_VSCROLL, state.state()),
            Self::EpEdittext(state) => (windows_sys::Win32::UI::Controls::EP_EDITTEXT, state.state()),
            Self::EpPassword(state) => (windows_sys::Win32::UI::Controls::EP_PASSWORD, state.state()),
            Self::EbpHeaderbackground(state) => (windows_sys::Win32::UI::Controls::EBP_HEADERBACKGROUND, state.state()),
            Self::EbpHeaderclose(state) => (windows_sys::Win32::UI::Controls::EBP_HEADERCLOSE, state.state()),
            Self::EbpHeaderpin(state) => (windows_sys::Win32::UI::Controls::EBP_HEADERPIN, state.state()),
            Self::EbpIebarmenu(state) => (windows_sys::Win32::UI::Controls::EBP_IEBARMENU, state.state()),
            Self::EbpNormalgroupbackground(state) => (windows_sys::Win32::UI::Controls::EBP_NORMALGROUPBACKGROUND, state.state()),
            Self::EbpNormalgroupcollapse(state) => (windows_sys::Win32::UI::Controls::EBP_NORMALGROUPCOLLAPSE, state.state()),
            Self::EbpNormalgroupexpand(state) => (windows_sys::Win32::UI::Controls::EBP_NORMALGROUPEXPAND, state.state()),
            Self::EbpNormalgrouphead(state) => (windows_sys::Win32::UI::Controls::EBP_NORMALGROUPHEAD, state.state()),
            Self::EbpSpecialgroupbackground(state) => (windows_sys::Win32::UI::Controls::EBP_SPECIALGROUPBACKGROUND, state.state()),
            Self::EbpSpecialgroupcollapse(state) => (windows_sys::Win32::UI::Controls::EBP_SPECIALGROUPCOLLAPSE, state.state()),
            Self::EbpSpecialgroupexpand(state) => (windows_sys::Win32::UI::Controls::EBP_SPECIALGROUPEXPAND, state.state()),
            Self::EbpSpecialgrouphead(state) => (windows_sys::Win32::UI::Controls::EBP_SPECIALGROUPHEAD, state.state()),
            Self::FlyoutBody(state) => (windows_sys::Win32::UI::Controls::FLYOUT_BODY, state.state()),
            Self::FlyoutDivider(state) => (windows_sys::Win32::UI::Controls::FLYOUT_DIVIDER, state.state()),
            Self::FlyoutHeader(state) => (windows_sys::Win32::UI::Controls::FLYOUT_HEADER, state.state()),
            Self::FlyoutLabel(state) => (windows_sys::Win32::UI::Controls::FLYOUT_LABEL, state.state()),
            Self::FlyoutLink(state) => (windows_sys::Win32::UI::Controls::FLYOUT_LINK, state.state()),
            Self::FlyoutLinkarea(state) => (windows_sys::Win32::UI::Controls::FLYOUT_LINKAREA, state.state()),
            Self::FlyoutLinkheader(state) => (windows_sys::Win32::UI::Controls::FLYOUT_LINKHEADER, state.state()),
            Self::FlyoutWindow(state) => (windows_sys::Win32::UI::Controls::FLYOUT_WINDOW, state.state()),
            Self::HpHeaderdropdown(state) => (windows_sys::Win32::UI::Controls::HP_HEADERDROPDOWN, state.state()),
            Self::HpHeaderdropdownfilter(state) => (windows_sys::Win32::UI::Controls::HP_HEADERDROPDOWNFILTER, state.state()),
            Self::HpHeaderitem(state) => (windows_sys::Win32::UI::Controls::HP_HEADERITEM, state.state()),
            Self::HpHeaderitemleft(state) => (windows_sys::Win32::UI::Controls::HP_HEADERITEMLEFT, state.state()),
            Self::HpHeaderitemright(state) => (windows_sys::Win32::UI::Controls::HP_HEADERITEMRIGHT, state.state()),
            Self::HpHeaderoverflow(state) => (windows_sys::Win32::UI::Controls::HP_HEADEROVERFLOW, state.state()),
            Self::HpHeadersortarrow(state) => (windows_sys::Win32::UI::Controls::HP_HEADERSORTARROW, state.state()),
            Self::LbcpBorderHscroll(state) => (windows_sys::Win32::UI::Controls::LBCP_BORDER_HSCROLL, state.state()),
            Self::LbcpBorderHvscroll(state) => (windows_sys::Win32::UI::Controls::LBCP_BORDER_HVSCROLL, state.state()),
            Self::LbcpBorderNoscroll(state) => (windows_sys::Win32::UI::Controls::LBCP_BORDER_NOSCROLL, state.state()),
            Self::LbcpBorderVscroll(state) => (windows_sys::Win32::UI::Controls::LBCP_BORDER_VSCROLL, state.state()),
            Self::LbcpItem(state) => (windows_sys::Win32::UI::Controls::LBCP_ITEM, state.state()),
            Self::LvpCollapsebutton(state) => (windows_sys::Win32::UI::Controls::LVP_COLLAPSEBUTTON, state.state()),
            Self::LvpColumndetail(state) => (windows_sys::Win32::UI::Controls::LVP_COLUMNDETAIL, state.state()),
            Self::LvpEmptytext(state) => (windows_sys::Win32::UI::Controls::LVP_EMPTYTEXT, state.state()),
            Self::LvpExpandbutton(state) => (windows_sys::Win32::UI::Controls::LVP_EXPANDBUTTON, state.state()),
            Self::LvpGroupheader(state) => (windows_sys::Win32::UI::Controls::LVP_GROUPHEADER, state.state()),
            Self::LvpGroupheaderline(state) => (windows_sys::Win32::UI::Controls::LVP_GROUPHEADERLINE, state.state()),
            Self::LvpListgroup(state) => (windows_sys::Win32::UI::Controls::LVP_LISTGROUP, state.state()),
            Self::LvpListdetail(state) => (windows_sys::Win32::UI::Controls::LVP_LISTDETAIL, state.state()),
            Self::LvpListitem(state) => (windows_sys::Win32::UI::Controls::LVP_LISTITEM, state.state()),
            Self::LvpListsorteddetail(state) => (windows_sys::Win32::UI::Controls::LVP_LISTSORTEDDETAIL, state.state()),
            Self::MenuBarbackground(state) => (windows_sys::Win32::UI::Controls::MENU_BARBACKGROUND, state.state()),
            Self::MenuBaritem(state) => (windows_sys::Win32::UI::Controls::MENU_BARITEM, state.state()),
            Self::MenuChevronTmschema(state) => (windows_sys::Win32::UI::Controls::MENU_CHEVRON_TMSCHEMA, state.state()),
            Self::MenuMenubardropdownTmschema(state) => (windows_sys::Win32::UI::Controls::MENU_MENUBARDROPDOWN_TMSCHEMA, state.state()),
            Self::MenuMenubaritemTmschema(state) => (windows_sys::Win32::UI::Controls::MENU_MENUBARITEM_TMSCHEMA, state.state()),
            Self::MenuMenudropdownTmschema(state) => (windows_sys::Win32::UI::Controls::MENU_MENUDROPDOWN_TMSCHEMA, state.state()),
            Self::MenuMenuitemTmschema(state) => (windows_sys::Win32::UI::Controls::MENU_MENUITEM_TMSCHEMA, state.state()),
            Self::MenuPopupbackground(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPBACKGROUND, state.state()),
            Self::MenuPopupborders(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPBORDERS, state.state()),
            Self::MenuPopupcheck(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPCHECK, state.state()),
            Self::MenuPopupcheckbackground(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPCHECKBACKGROUND, state.state()),
            Self::MenuPopupgutter(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPGUTTER, state.state()),
            Self::MenuPopupitem(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPITEM, state.state()),
            Self::MenuPopupseparator(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPSEPARATOR, state.state()),
            Self::MenuPopupsubmenu(state) => (windows_sys::Win32::UI::Controls::MENU_POPUPSUBMENU, state.state()),
            Self::MenuSeparatorTmschema(state) => (windows_sys::Win32::UI::Controls::MENU_SEPARATOR_TMSCHEMA, state.state()),
            Self::MenuSystemclose(state) => (windows_sys::Win32::UI::Controls::MENU_SYSTEMCLOSE, state.state()),
            Self::MenuSystemmaximize(state) => (windows_sys::Win32::UI::Controls::MENU_SYSTEMMAXIMIZE, state.state()),
            Self::MenuSystemminimize(state) => (windows_sys::Win32::UI::Controls::MENU_SYSTEMMINIMIZE, state.state()),
            Self::MenuSystemrestore(state) => (windows_sys::Win32::UI::Controls::MENU_SYSTEMRESTORE, state.state()),
            Self::MdpNewappbutton(state) => (windows_sys::Win32::UI::Controls::MDP_NEWAPPBUTTON, state.state()),
            Self::MdpSeperator(state) => (windows_sys::Win32::UI::Controls::MDP_SEPERATOR, state.state()),
            Self::NavBackbutton(state) => (windows_sys::Win32::UI::Controls::NAV_BACKBUTTON, state.state()),
            Self::NavForwardbutton(state) => (windows_sys::Win32::UI::Controls::NAV_FORWARDBUTTON, state.state()),
            Self::NavMenubutton(state) => (windows_sys::Win32::UI::Controls::NAV_MENUBUTTON, state.state()),
            Self::PgrpDown(state) => (windows_sys::Win32::UI::Controls::PGRP_DOWN, state.state()),
            Self::PgrpDownhorz(state) => (windows_sys::Win32::UI::Controls::PGRP_DOWNHORZ, state.state()),
            Self::PgrpUp(state) => (windows_sys::Win32::UI::Controls::PGRP_UP, state.state()),
            Self::PgrpUphorz(state) => (windows_sys::Win32::UI::Controls::PGRP_UPHORZ, state.state()),
            Self::PpBar(state) => (windows_sys::Win32::UI::Controls::PP_BAR, state.state()),
            Self::PpBarvert(state) => (windows_sys::Win32::UI::Controls::PP_BARVERT, state.state()),
            Self::PpChunk(state) => (windows_sys::Win32::UI::Controls::PP_CHUNK, state.state()),
            Self::PpChunkvert(state) => (windows_sys::Win32::UI::Controls::PP_CHUNKVERT, state.state()),
            Self::PpFill(state) => (windows_sys::Win32::UI::Controls::PP_FILL, state.state()),
            Self::PpFillvert(state) => (windows_sys::Win32::UI::Controls::PP_FILLVERT, state.state()),
            Self::PpMoveoverlay(state) => (windows_sys::Win32::UI::Controls::PP_MOVEOVERLAY, state.state()),
            Self::PpMoveoverlayvert(state) => (windows_sys::Win32::UI::Controls::PP_MOVEOVERLAYVERT, state.state()),
            Self::PpPulseoverlay(state) => (windows_sys::Win32::UI::Controls::PP_PULSEOVERLAY, state.state()),
            Self::PpPulseoverlayvert(state) => (windows_sys::Win32::UI::Controls::PP_PULSEOVERLAYVERT, state.state()),
            Self::PpTransparentbar(state) => (windows_sys::Win32::UI::Controls::PP_TRANSPARENTBAR, state.state()),
            Self::PpTransparentbarvert(state) => (windows_sys::Win32::UI::Controls::PP_TRANSPARENTBARVERT, state.state()),
            Self::RpBackground(state) => (windows_sys::Win32::UI::Controls::RP_BACKGROUND, state.state()),
            Self::RpBand(state) => (windows_sys::Win32::UI::Controls::RP_BAND, state.state()),
            Self::RpChevron(state) => (windows_sys::Win32::UI::Controls::RP_CHEVRON, state.state()),
            Self::RpChevronvert(state) => (windows_sys::Win32::UI::Controls::RP_CHEVRONVERT, state.state()),
            Self::RpGripper(state) => (windows_sys::Win32::UI::Controls::RP_GRIPPER, state.state()),
            Self::RpGrippervert(state) => (windows_sys::Win32::UI::Controls::RP_GRIPPERVERT, state.state()),
            Self::RpSplitter(state) => (windows_sys::Win32::UI::Controls::RP_SPLITTER, state.state()),
            Self::RpSplittervert(state) => (windows_sys::Win32::UI::Controls::RP_SPLITTERVERT, state.state()),
            Self::SbpArrowbtn(state) => (windows_sys::Win32::UI::Controls::SBP_ARROWBTN, state.state()),
            Self::SbpGripperhorz(state) => (windows_sys::Win32::UI::Controls::SBP_GRIPPERHORZ, state.state()),
            Self::SbpGrippervert(state) => (windows_sys::Win32::UI::Controls::SBP_GRIPPERVERT, state.state()),
            Self::SbpLowertrackhorz(state) => (windows_sys::Win32::UI::Controls::SBP_LOWERTRACKHORZ, state.state()),
            Self::SbpLowertrackvert(state) => (windows_sys::Win32::UI::Controls::SBP_LOWERTRACKVERT, state.state()),
            Self::SbpSizebox(state) => (windows_sys::Win32::UI::Controls::SBP_SIZEBOX, state.state()),
            Self::SbpThumbbtnhorz(state) => (windows_sys::Win32::UI::Controls::SBP_THUMBBTNHORZ, state.state()),
            Self::SbpThumbbtnvert(state) => (windows_sys::Win32::UI::Controls::SBP_THUMBBTNVERT, state.state()),
            Self::SbpUppertrackhorz(state) => (windows_sys::Win32::UI::Controls::SBP_UPPERTRACKHORZ, state.state()),
            Self::SbpUppertrackvert(state) => (windows_sys::Win32::UI::Controls::SBP_UPPERTRACKVERT, state.state()),
            Self::SpnpDown(state) => (windows_sys::Win32::UI::Controls::SPNP_DOWN, state.state()),
            Self::SpnpDownhorz(state) => (windows_sys::Win32::UI::Controls::SPNP_DOWNHORZ, state.state()),
            Self::SpnpUp(state) => (windows_sys::Win32::UI::Controls::SPNP_UP, state.state()),
            Self::SpnpUphorz(state) => (windows_sys::Win32::UI::Controls::SPNP_UPHORZ, state.state()),
            Self::SppLogoff(state) => (windows_sys::Win32::UI::Controls::SPP_LOGOFF, state.state()),
            Self::SppLogoffbuttons(state) => (windows_sys::Win32::UI::Controls::SPP_LOGOFFBUTTONS, state.state()),
            Self::SppMoreprograms(state) => (windows_sys::Win32::UI::Controls::SPP_MOREPROGRAMS, state.state()),
            Self::SppMoreprogramsarrow(state) => (windows_sys::Win32::UI::Controls::SPP_MOREPROGRAMSARROW, state.state()),
            Self::SppPlaceslist(state) => (windows_sys::Win32::UI::Controls::SPP_PLACESLIST, state.state()),
            Self::SppPlaceslistseparator(state) => (windows_sys::Win32::UI::Controls::SPP_PLACESLISTSEPARATOR, state.state()),
            Self::SppPreview(state) => (windows_sys::Win32::UI::Controls::SPP_PREVIEW, state.state()),
            Self::SppProglist(state) => (windows_sys::Win32::UI::Controls::SPP_PROGLIST, state.state()),
            Self::SppProglistseparator(state) => (windows_sys::Win32::UI::Controls::SPP_PROGLISTSEPARATOR, state.state()),
            Self::SppUserpane(state) => (windows_sys::Win32::UI::Controls::SPP_USERPANE, state.state()),
            Self::SppUserpicture(state) => (windows_sys::Win32::UI::Controls::SPP_USERPICTURE, state.state()),
            Self::SpGripper(state) => (windows_sys::Win32::UI::Controls::SP_GRIPPER, state.state()),
            Self::SpGripperpane(state) => (windows_sys::Win32::UI::Controls::SP_GRIPPERPANE, state.state()),
            Self::SpPane(state) => (windows_sys::Win32::UI::Controls::SP_PANE, state.state()),
            Self::TabpAerowizardbody(state) => (windows_sys::Win32::UI::Controls::TABP_AEROWIZARDBODY, state.state()),
            Self::TabpBody(state) => (windows_sys::Win32::UI::Controls::TABP_BODY, state.state()),
            Self::TabpPane(state) => (windows_sys::Win32::UI::Controls::TABP_PANE, state.state()),
            Self::TabpTabitem(state) => (windows_sys::Win32::UI::Controls::TABP_TABITEM, state.state()),
            Self::TabpTabitembothedge(state) => (windows_sys::Win32::UI::Controls::TABP_TABITEMBOTHEDGE, state.state()),
            Self::TabpTabitemleftedge(state) => (windows_sys::Win32::UI::Controls::TABP_TABITEMLEFTEDGE, state.state()),
            Self::TabpTabitemrightedge(state) => (windows_sys::Win32::UI::Controls::TABP_TABITEMRIGHTEDGE, state.state()),
            Self::TabpToptabitem(state) => (windows_sys::Win32::UI::Controls::TABP_TOPTABITEM, state.state()),
            Self::TabpToptabitembothedge(state) => (windows_sys::Win32::UI::Controls::TABP_TOPTABITEMBOTHEDGE, state.state()),
            Self::TabpToptabitemleftedge(state) => (windows_sys::Win32::UI::Controls::TABP_TOPTABITEMLEFTEDGE, state.state()),
            Self::TabpToptabitemrightedge(state) => (windows_sys::Win32::UI::Controls::TABP_TOPTABITEMRIGHTEDGE, state.state()),
            Self::TdpGroupcount(state) => (windows_sys::Win32::UI::Controls::TDP_GROUPCOUNT, state.state()),
            Self::TdpFlashbutton(state) => (windows_sys::Win32::UI::Controls::TDP_FLASHBUTTON, state.state()),
            Self::TdpFlashbuttongroupmenu(state) => (windows_sys::Win32::UI::Controls::TDP_FLASHBUTTONGROUPMENU, state.state()),
            Self::TbpBackgroundbottom(state) => (windows_sys::Win32::UI::Controls::TBP_BACKGROUNDBOTTOM, state.state()),
            Self::TbpBackgroundleft(state) => (windows_sys::Win32::UI::Controls::TBP_BACKGROUNDLEFT, state.state()),
            Self::TbpBackgroundright(state) => (windows_sys::Win32::UI::Controls::TBP_BACKGROUNDRIGHT, state.state()),
            Self::TbpBackgroundtop(state) => (windows_sys::Win32::UI::Controls::TBP_BACKGROUNDTOP, state.state()),
            Self::TbpSizingbarbottom(state) => (windows_sys::Win32::UI::Controls::TBP_SIZINGBARBOTTOM, state.state()),
            Self::TbpSizingbarright(state) => (windows_sys::Win32::UI::Controls::TBP_SIZINGBARRIGHT, state.state()),
            Self::TbpSizingbartop(state) => (windows_sys::Win32::UI::Controls::TBP_SIZINGBARTOP, state.state()),
            Self::TdlgButtonsection(state) => (windows_sys::Win32::UI::Controls::TDLG_BUTTONSECTION, state.state()),
            Self::TdlgButtonwrapper(state) => (windows_sys::Win32::UI::Controls::TDLG_BUTTONWRAPPER, state.state()),
            Self::TdlgCommandlinkpane(state) => (windows_sys::Win32::UI::Controls::TDLG_COMMANDLINKPANE, state.state()),
            Self::TdlgContenticon(state) => (windows_sys::Win32::UI::Controls::TDLG_CONTENTICON, state.state()),
            Self::TdlgContentpane(state) => (windows_sys::Win32::UI::Controls::TDLG_CONTENTPANE, state.state()),
            Self::TdlgControlpane(state) => (windows_sys::Win32::UI::Controls::TDLG_CONTROLPANE, state.state()),
            Self::TdlgExpandedcontent(state) => (windows_sys::Win32::UI::Controls::TDLG_EXPANDEDCONTENT, state.state()),
            Self::TdlgExpandedfooterarea(state) => (windows_sys::Win32::UI::Controls::TDLG_EXPANDEDFOOTERAREA, state.state()),
            Self::TdlgExpandobutton(state) => (windows_sys::Win32::UI::Controls::TDLG_EXPANDOBUTTON, state.state()),
            Self::TdlgExpandotext(state) => (windows_sys::Win32::UI::Controls::TDLG_EXPANDOTEXT, state.state()),
            Self::TdlgFootnotearea(state) => (windows_sys::Win32::UI::Controls::TDLG_FOOTNOTEAREA, state.state()),
            Self::TdlgFootnotepane(state) => (windows_sys::Win32::UI::Controls::TDLG_FOOTNOTEPANE, state.state()),
            Self::TdlgFootnoteseparator(state) => (windows_sys::Win32::UI::Controls::TDLG_FOOTNOTESEPARATOR, state.state()),
            Self::TdlgImagealignment(state) => (windows_sys::Win32::UI::Controls::TDLG_IMAGEALIGNMENT, state.state()),
            Self::TdlgMainicon(state) => (windows_sys::Win32::UI::Controls::TDLG_MAINICON, state.state()),
            Self::TdlgMaininstructionpane(state) => (windows_sys::Win32::UI::Controls::TDLG_MAININSTRUCTIONPANE, state.state()),
            Self::TdlgPrimarypanel(state) => (windows_sys::Win32::UI::Controls::TDLG_PRIMARYPANEL, state.state()),
            Self::TdlgProgressbar(state) => (windows_sys::Win32::UI::Controls::TDLG_PROGRESSBAR, state.state()),
            Self::TdlgRadiobuttonpane(state) => (windows_sys::Win32::UI::Controls::TDLG_RADIOBUTTONPANE, state.state()),
            Self::TdlgSecondarypanel(state) => (windows_sys::Win32::UI::Controls::TDLG_SECONDARYPANEL, state.state()),
            Self::TdlgVerificationtext(state) => (windows_sys::Win32::UI::Controls::TDLG_VERIFICATIONTEXT, state.state()),
            Self::TextBodytitle(state) => (windows_sys::Win32::UI::Controls::TEXT_BODYTITLE, state.state()),
            Self::TextBodytext(state) => (windows_sys::Win32::UI::Controls::TEXT_BODYTEXT, state.state()),
            Self::TextControllabel(state) => (windows_sys::Win32::UI::Controls::TEXT_CONTROLLABEL, state.state()),
            Self::TextExpanded(state) => (windows_sys::Win32::UI::Controls::TEXT_EXPANDED, state.state()),
            Self::TextHyperlinktext(state) => (windows_sys::Win32::UI::Controls::TEXT_HYPERLINKTEXT, state.state()),
            Self::TextInstruction(state) => (windows_sys::Win32::UI::Controls::TEXT_INSTRUCTION, state.state()),
            Self::TextLabel(state) => (windows_sys::Win32::UI::Controls::TEXT_LABEL, state.state()),
            Self::TextMaininstruction(state) => (windows_sys::Win32::UI::Controls::TEXT_MAININSTRUCTION, state.state()),
            Self::TextSecondarytext(state) => (windows_sys::Win32::UI::Controls::TEXT_SECONDARYTEXT, state.state()),
            Self::TpButton(state) => (windows_sys::Win32::UI::Controls::TP_BUTTON, state.state()),
            Self::TpDropdownbutton(state) => (windows_sys::Win32::UI::Controls::TP_DROPDOWNBUTTON, state.state()),
            Self::TpDropdownbuttonglyph(state) => (windows_sys::Win32::UI::Controls::TP_DROPDOWNBUTTONGLYPH, state.state()),
            Self::TpSeparator(state) => (windows_sys::Win32::UI::Controls::TP_SEPARATOR, state.state()),
            Self::TpSeparatorvert(state) => (windows_sys::Win32::UI::Controls::TP_SEPARATORVERT, state.state()),
            Self::TpSplitbutton(state) => (windows_sys::Win32::UI::Controls::TP_SPLITBUTTON, state.state()),
            Self::TpSplitbuttondropdown(state) => (windows_sys::Win32::UI::Controls::TP_SPLITBUTTONDROPDOWN, state.state()),
            Self::TtpBalloon(state) => (windows_sys::Win32::UI::Controls::TTP_BALLOON, state.state()),
            Self::TtpBalloonstem(state) => (windows_sys::Win32::UI::Controls::TTP_BALLOONSTEM, state.state()),
            Self::TtpBalloontitle(state) => (windows_sys::Win32::UI::Controls::TTP_BALLOONTITLE, state.state()),
            Self::TtpClose(state) => (windows_sys::Win32::UI::Controls::TTP_CLOSE, state.state()),
            Self::TtpStandard(state) => (windows_sys::Win32::UI::Controls::TTP_STANDARD, state.state()),
            Self::TtpStandardtitle(state) => (windows_sys::Win32::UI::Controls::TTP_STANDARDTITLE, state.state()),
            Self::TtpWrench(state) => (windows_sys::Win32::UI::Controls::TTP_WRENCH, state.state()),
            Self::TkpThumb(state) => (windows_sys::Win32::UI::Controls::TKP_THUMB, state.state()),
            Self::TkpThumbbottom(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBBOTTOM, state.state()),
            Self::TkpThumbleft(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBLEFT, state.state()),
            Self::TkpThumbright(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBRIGHT, state.state()),
            Self::TkpThumbtop(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBTOP, state.state()),
            Self::TkpThumbvert(state) => (windows_sys::Win32::UI::Controls::TKP_THUMBVERT, state.state()),
            Self::TkpTics(state) => (windows_sys::Win32::UI::Controls::TKP_TICS, state.state()),
            Self::TkpTicsvert(state) => (windows_sys::Win32::UI::Controls::TKP_TICSVERT, state.state()),
            Self::TkpTrack(state) => (windows_sys::Win32::UI::Controls::TKP_TRACK, state.state()),
            Self::TkpTrackvert(state) => (windows_sys::Win32::UI::Controls::TKP_TRACKVERT, state.state()),
            Self::TnpAnimbackground(state) => (windows_sys::Win32::UI::Controls::TNP_ANIMBACKGROUND, state.state()),
            Self::TnpBackground(state) => (windows_sys::Win32::UI::Controls::TNP_BACKGROUND, state.state()),
            Self::TvpBranch(state) => (windows_sys::Win32::UI::Controls::TVP_BRANCH, state.state()),
            Self::TvpGlyph(state) => (windows_sys::Win32::UI::Controls::TVP_GLYPH, state.state()),
            Self::TvpHotglyph(state) => (windows_sys::Win32::UI::Controls::TVP_HOTGLYPH, state.state()),
            Self::TvpTreeitem(state) => (windows_sys::Win32::UI::Controls::TVP_TREEITEM, state.state()),
            Self::WpCaption(state) => (windows_sys::Win32::UI::Controls::WP_CAPTION, state.state()),
            Self::WpCaptionsizingtemplate(state) => (windows_sys::Win32::UI::Controls::WP_CAPTIONSIZINGTEMPLATE, state.state()),
            Self::WpClosebutton(state) => (windows_sys::Win32::UI::Controls::WP_CLOSEBUTTON, state.state()),
            Self::WpDialog(state) => (windows_sys::Win32::UI::Controls::WP_DIALOG, state.state()),
            Self::WpFrame(state) => (windows_sys::Win32::UI::Controls::WP_FRAME, state.state()),
            Self::WpFramebottom(state) => (windows_sys::Win32::UI::Controls::WP_FRAMEBOTTOM, state.state()),
            Self::WpFramebottomsizingtemplate(state) => (windows_sys::Win32::UI::Controls::WP_FRAMEBOTTOMSIZINGTEMPLATE, state.state()),
            Self::WpFrameleft(state) => (windows_sys::Win32::UI::Controls::WP_FRAMELEFT, state.state()),
            Self::WpFrameleftsizingtemplate(state) => (windows_sys::Win32::UI::Controls::WP_FRAMELEFTSIZINGTEMPLATE, state.state()),
            Self::WpFrameright(state) => (windows_sys::Win32::UI::Controls::WP_FRAMERIGHT, state.state()),
            Self::WpFramerightsizingtemplate(state) => (windows_sys::Win32::UI::Controls::WP_FRAMERIGHTSIZINGTEMPLATE, state.state()),
            Self::WpHelpbutton(state) => (windows_sys::Win32::UI::Controls::WP_HELPBUTTON, state.state()),
            Self::WpHorzscroll(state) => (windows_sys::Win32::UI::Controls::WP_HORZSCROLL, state.state()),
            Self::WpHorzthumb(state) => (windows_sys::Win32::UI::Controls::WP_HORZTHUMB, state.state()),
            Self::WpMaxbutton(state) => (windows_sys::Win32::UI::Controls::WP_MAXBUTTON, state.state()),
            Self::WpMaxcaption(state) => (windows_sys::Win32::UI::Controls::WP_MAXCAPTION, state.state()),
            Self::WpMdiclosebutton(state) => (windows_sys::Win32::UI::Controls::WP_MDICLOSEBUTTON, state.state()),
            Self::WpMdihelpbutton(state) => (windows_sys::Win32::UI::Controls::WP_MDIHELPBUTTON, state.state()),
            Self::WpMdiminbutton(state) => (windows_sys::Win32::UI::Controls::WP_MDIMINBUTTON, state.state()),
            Self::WpMdirestorebutton(state) => (windows_sys::Win32::UI::Controls::WP_MDIRESTOREBUTTON, state.state()),
            Self::WpMdisysbutton(state) => (windows_sys::Win32::UI::Controls::WP_MDISYSBUTTON, state.state()),
            Self::WpMinbutton(state) => (windows_sys::Win32::UI::Controls::WP_MINBUTTON, state.state()),
            Self::WpMincaption(state) => (windows_sys::Win32::UI::Controls::WP_MINCAPTION, state.state()),
            Self::WpRestorebutton(state) => (windows_sys::Win32::UI::Controls::WP_RESTOREBUTTON, state.state()),
            Self::WpSmallcaption(state) => (windows_sys::Win32::UI::Controls::WP_SMALLCAPTION, state.state()),
            Self::WpSmallcaptionsizingtemplate(state) => (windows_sys::Win32::UI::Controls::WP_SMALLCAPTIONSIZINGTEMPLATE, state.state()),
            Self::WpSmallclosebutton(state) => (windows_sys::Win32::UI::Controls::WP_SMALLCLOSEBUTTON, state.state()),
            Self::WpSmallframebottom(state) => (windows_sys::Win32::UI::Controls::WP_SMALLFRAMEBOTTOM, state.state()),
            Self::WpSmallframebottomsizingtemplate(state) => (windows_sys::Win32::UI::Controls::WP_SMALLFRAMEBOTTOMSIZINGTEMPLATE, state.state()),
            Self::WpSmallframeleft(state) => (windows_sys::Win32::UI::Controls::WP_SMALLFRAMELEFT, state.state()),
            Self::WpSmallframeleftsizingtemplate(state) => (windows_sys::Win32::UI::Controls::WP_SMALLFRAMELEFTSIZINGTEMPLATE, state.state()),
            Self::WpSmallframeright(state) => (windows_sys::Win32::UI::Controls::WP_SMALLFRAMERIGHT, state.state()),
            Self::WpSmallframerightsizingtemplate(state) => (windows_sys::Win32::UI::Controls::WP_SMALLFRAMERIGHTSIZINGTEMPLATE, state.state()),
            Self::WpSmallmaxcaption(state) => (windows_sys::Win32::UI::Controls::WP_SMALLMAXCAPTION, state.state()),
            Self::WpSmallmincaption(state) => (windows_sys::Win32::UI::Controls::WP_SMALLMINCAPTION, state.state()),
            Self::WpSysbutton(state) => (windows_sys::Win32::UI::Controls::WP_SYSBUTTON, state.state()),
            Self::WpVertscroll(state) => (windows_sys::Win32::UI::Controls::WP_VERTSCROLL, state.state()),
            Self::WpVertthumb(state) => (windows_sys::Win32::UI::Controls::WP_VERTTHUMB, state.state()),
        }
    }
}

impl crate::Theme {
    /// Gets the `TMT_BOOL` property.
    #[inline]
    pub fn tmt_bool(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_BOOL)
    }

    /// Gets the `TMT_ALWAYSSHOWSIZINGBAR` property.
    #[inline]
    pub fn tmt_alwaysshowsizingbar(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_ALWAYSSHOWSIZINGBAR)
    }

    /// Gets the `TMT_AUTOSIZE` property.
    #[inline]
    pub fn tmt_autosize(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_AUTOSIZE)
    }

    /// Gets the `TMT_BGFILL` property.
    #[inline]
    pub fn tmt_bgfill(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_BGFILL)
    }

    /// Gets the `TMT_BORDERONLY` property.
    #[inline]
    pub fn tmt_borderonly(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_BORDERONLY)
    }

    /// Gets the `TMT_COMPOSITED` property.
    #[inline]
    pub fn tmt_composited(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_COMPOSITED)
    }

    /// Gets the `TMT_COMPOSITEDOPAQUE` property.
    #[inline]
    pub fn tmt_compositedopaque(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_COMPOSITEDOPAQUE)
    }

    /// Gets the `TMT_DRAWBORDERS` property.
    #[inline]
    pub fn tmt_drawborders(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_DRAWBORDERS)
    }

    /// Gets the `TMT_FLATMENUS` property.
    #[inline]
    pub fn tmt_flatmenus(&self) -> crate::GetThemeBoolRet {
        self.get_theme_sys_bool(windows_sys::Win32::UI::Controls::TMT_FLATMENUS)
    }

    /// Gets the `TMT_GLYPHONLY` property.
    #[inline]
    pub fn tmt_glyphonly(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_GLYPHONLY)
    }

    /// Gets the `TMT_GLYPHTRANSPARENT` property.
    #[inline]
    pub fn tmt_glyphtransparent(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_GLYPHTRANSPARENT)
    }

    /// Gets the `TMT_INTEGRALSIZING` property.
    #[inline]
    pub fn tmt_integralsizing(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_INTEGRALSIZING)
    }

    /// Gets the `TMT_LOCALIZEDMIRRORIMAGE` property.
    #[inline]
    pub fn tmt_localizedmirrorimage(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_LOCALIZEDMIRRORIMAGE)
    }

    /// Gets the `TMT_MIRRORIMAGE` property.
    #[inline]
    pub fn tmt_mirrorimage(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_MIRRORIMAGE)
    }

    /// Gets the `TMT_NOETCHEDEFFECT` property.
    #[inline]
    pub fn tmt_noetchedeffect(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_NOETCHEDEFFECT)
    }

    /// Gets the `TMT_SCALEDBACKGROUND` property.
    #[inline]
    pub fn tmt_scaledbackground(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_SCALEDBACKGROUND)
    }

    /// Gets the `TMT_SOURCEGROW` property.
    #[inline]
    pub fn tmt_sourcegrow(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_SOURCEGROW)
    }

    /// Gets the `TMT_SOURCESHRINK` property.
    #[inline]
    pub fn tmt_sourceshrink(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_SOURCESHRINK)
    }

    /// Gets the `TMT_TEXTAPPLYOVERLAY` property.
    #[inline]
    pub fn tmt_textapplyoverlay(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_TEXTAPPLYOVERLAY)
    }

    /// Gets the `TMT_TEXTGLOW` property.
    #[inline]
    pub fn tmt_textglow(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_TEXTGLOW)
    }

    /// Gets the `TMT_TEXTITALIC` property.
    #[inline]
    pub fn tmt_textitalic(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_TEXTITALIC)
    }

    /// Gets the `TMT_TRANSPARENT` property.
    #[inline]
    pub fn tmt_transparent(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_TRANSPARENT)
    }

    /// Gets the `TMT_UNIFORMSIZING` property.
    #[inline]
    pub fn tmt_uniformsizing(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_UNIFORMSIZING)
    }

    /// Gets the `TMT_USERPICTURE` property.
    #[inline]
    pub fn tmt_userpicture(&self, part: Part) -> crate::GetThemeBoolRet {
        self.get_theme_bool(part, windows_sys::Win32::UI::Controls::TMT_USERPICTURE)
    }

    /// Gets the `TMT_COLOR` property.
    #[inline]
    pub fn tmt_color(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_COLOR)
    }

    /// Gets the `TMT_ACCENTCOLORHINT` property.
    #[inline]
    pub fn tmt_accentcolorhint(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_ACCENTCOLORHINT)
    }

    /// Gets the `TMT_ACTIVEBORDER` property.
    #[inline]
    pub fn tmt_activeborder(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_ACTIVEBORDER)
    }

    /// Gets the `TMT_ACTIVECAPTION` property.
    #[inline]
    pub fn tmt_activecaption(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_ACTIVECAPTION)
    }

    /// Gets the `TMT_APPWORKSPACE` property.
    #[inline]
    pub fn tmt_appworkspace(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_APPWORKSPACE)
    }

    /// Gets the `TMT_BACKGROUND` property.
    #[inline]
    pub fn tmt_background(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BACKGROUND)
    }

    /// Gets the `TMT_BLENDCOLOR` property.
    #[inline]
    pub fn tmt_blendcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BLENDCOLOR)
    }

    /// Gets the `TMT_BODYTEXTCOLOR` property.
    #[inline]
    pub fn tmt_bodytextcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BODYTEXTCOLOR)
    }

    /// Gets the `TMT_BORDERCOLOR` property.
    #[inline]
    pub fn tmt_bordercolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BORDERCOLOR)
    }

    /// Gets the `TMT_BORDERCOLORHINT` property.
    #[inline]
    pub fn tmt_bordercolorhint(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BORDERCOLORHINT)
    }

    /// Gets the `TMT_BTNFACE` property.
    #[inline]
    pub fn tmt_btnface(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BTNFACE)
    }

    /// Gets the `TMT_BTNHIGHLIGHT` property.
    #[inline]
    pub fn tmt_btnhighlight(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BTNHIGHLIGHT)
    }

    /// Gets the `TMT_BTNSHADOW` property.
    #[inline]
    pub fn tmt_btnshadow(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BTNSHADOW)
    }

    /// Gets the `TMT_BTNTEXT` property.
    #[inline]
    pub fn tmt_btntext(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BTNTEXT)
    }

    /// Gets the `TMT_BUTTONALTERNATEFACE` property.
    #[inline]
    pub fn tmt_buttonalternateface(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_BUTTONALTERNATEFACE)
    }

    /// Gets the `TMT_CAPTIONTEXT` property.
    #[inline]
    pub fn tmt_captiontext(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_CAPTIONTEXT)
    }

    /// Gets the `TMT_DKSHADOW3D` property.
    #[inline]
    pub fn tmt_dkshadow3d(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_DKSHADOW3D)
    }

    /// Gets the `TMT_EDGEDKSHADOWCOLOR` property.
    #[inline]
    pub fn tmt_edgedkshadowcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_EDGEDKSHADOWCOLOR)
    }

    /// Gets the `TMT_EDGEFILLCOLOR` property.
    #[inline]
    pub fn tmt_edgefillcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_EDGEFILLCOLOR)
    }

    /// Gets the `TMT_EDGEHIGHLIGHTCOLOR` property.
    #[inline]
    pub fn tmt_edgehighlightcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_EDGEHIGHLIGHTCOLOR)
    }

    /// Gets the `TMT_EDGELIGHTCOLOR` property.
    #[inline]
    pub fn tmt_edgelightcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_EDGELIGHTCOLOR)
    }

    /// Gets the `TMT_EDGESHADOWCOLOR` property.
    #[inline]
    pub fn tmt_edgeshadowcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_EDGESHADOWCOLOR)
    }

    /// Gets the `TMT_FILLCOLOR` property.
    #[inline]
    pub fn tmt_fillcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_FILLCOLOR)
    }

    /// Gets the `TMT_FILLCOLORHINT` property.
    #[inline]
    pub fn tmt_fillcolorhint(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_FILLCOLORHINT)
    }

    /// Gets the `TMT_FROMCOLOR1` property.
    #[inline]
    pub fn tmt_fromcolor1(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_FROMCOLOR1)
    }

    /// Gets the `TMT_FROMCOLOR2` property.
    #[inline]
    pub fn tmt_fromcolor2(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_FROMCOLOR2)
    }

    /// Gets the `TMT_FROMCOLOR3` property.
    #[inline]
    pub fn tmt_fromcolor3(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_FROMCOLOR3)
    }

    /// Gets the `TMT_FROMCOLOR4` property.
    #[inline]
    pub fn tmt_fromcolor4(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_FROMCOLOR4)
    }

    /// Gets the `TMT_FROMCOLOR5` property.
    #[inline]
    pub fn tmt_fromcolor5(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_FROMCOLOR5)
    }

    /// Gets the `TMT_GLOWCOLOR` property.
    #[inline]
    pub fn tmt_glowcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GLOWCOLOR)
    }

    /// Gets the `TMT_GLYPHTEXTCOLOR` property.
    #[inline]
    pub fn tmt_glyphtextcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GLYPHTEXTCOLOR)
    }

    /// Gets the `TMT_GLYPHTRANSPARENTCOLOR` property.
    #[inline]
    pub fn tmt_glyphtransparentcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GLYPHTRANSPARENTCOLOR)
    }

    /// Gets the `TMT_GRADIENTACTIVECAPTION` property.
    #[inline]
    pub fn tmt_gradientactivecaption(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTACTIVECAPTION)
    }

    /// Gets the `TMT_GRADIENTCOLOR1` property.
    #[inline]
    pub fn tmt_gradientcolor1(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTCOLOR1)
    }

    /// Gets the `TMT_GRADIENTCOLOR2` property.
    #[inline]
    pub fn tmt_gradientcolor2(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTCOLOR2)
    }

    /// Gets the `TMT_GRADIENTCOLOR3` property.
    #[inline]
    pub fn tmt_gradientcolor3(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTCOLOR3)
    }

    /// Gets the `TMT_GRADIENTCOLOR4` property.
    #[inline]
    pub fn tmt_gradientcolor4(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTCOLOR4)
    }

    /// Gets the `TMT_GRADIENTCOLOR5` property.
    #[inline]
    pub fn tmt_gradientcolor5(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTCOLOR5)
    }

    /// Gets the `TMT_GRADIENTINACTIVECAPTION` property.
    #[inline]
    pub fn tmt_gradientinactivecaption(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTINACTIVECAPTION)
    }

    /// Gets the `TMT_GRAYTEXT` property.
    #[inline]
    pub fn tmt_graytext(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_GRAYTEXT)
    }

    /// Gets the `TMT_HEADING1TEXTCOLOR` property.
    #[inline]
    pub fn tmt_heading1textcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_HEADING1TEXTCOLOR)
    }

    /// Gets the `TMT_HEADING2TEXTCOLOR` property.
    #[inline]
    pub fn tmt_heading2textcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_HEADING2TEXTCOLOR)
    }

    /// Gets the `TMT_HIGHLIGHT` property.
    #[inline]
    pub fn tmt_highlight(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_HIGHLIGHT)
    }

    /// Gets the `TMT_HIGHLIGHTTEXT` property.
    #[inline]
    pub fn tmt_highlighttext(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_HIGHLIGHTTEXT)
    }

    /// Gets the `TMT_HOTTRACKING` property.
    #[inline]
    pub fn tmt_hottracking(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_HOTTRACKING)
    }

    /// Gets the `TMT_INACTIVEBORDER` property.
    #[inline]
    pub fn tmt_inactiveborder(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_INACTIVEBORDER)
    }

    /// Gets the `TMT_INACTIVECAPTION` property.
    #[inline]
    pub fn tmt_inactivecaption(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_INACTIVECAPTION)
    }

    /// Gets the `TMT_INACTIVECAPTIONTEXT` property.
    #[inline]
    pub fn tmt_inactivecaptiontext(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_INACTIVECAPTIONTEXT)
    }

    /// Gets the `TMT_INFOBK` property.
    #[inline]
    pub fn tmt_infobk(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_INFOBK)
    }

    /// Gets the `TMT_INFOTEXT` property.
    #[inline]
    pub fn tmt_infotext(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_INFOTEXT)
    }

    /// Gets the `TMT_LIGHT3D` property.
    #[inline]
    pub fn tmt_light3d(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_LIGHT3D)
    }

    /// Gets the `TMT_MENU` property.
    #[inline]
    pub fn tmt_menu(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_MENU)
    }

    /// Gets the `TMT_MENUBAR` property.
    #[inline]
    pub fn tmt_menubar(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_MENUBAR)
    }

    /// Gets the `TMT_MENUHILIGHT` property.
    #[inline]
    pub fn tmt_menuhilight(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_MENUHILIGHT)
    }

    /// Gets the `TMT_MENUTEXT` property.
    #[inline]
    pub fn tmt_menutext(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_MENUTEXT)
    }

    /// Gets the `TMT_SCROLLBAR` property.
    #[inline]
    pub fn tmt_scrollbar(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_SCROLLBAR)
    }

    /// Gets the `TMT_SHADOWCOLOR` property.
    #[inline]
    pub fn tmt_shadowcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_SHADOWCOLOR)
    }

    /// Gets the `TMT_TEXTBORDERCOLOR` property.
    #[inline]
    pub fn tmt_textbordercolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_TEXTBORDERCOLOR)
    }

    /// Gets the `TMT_TEXTCOLOR` property.
    #[inline]
    pub fn tmt_textcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_TEXTCOLOR)
    }

    /// Gets the `TMT_TEXTCOLORHINT` property.
    #[inline]
    pub fn tmt_textcolorhint(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_TEXTCOLORHINT)
    }

    /// Gets the `TMT_TEXTSHADOWCOLOR` property.
    #[inline]
    pub fn tmt_textshadowcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_TEXTSHADOWCOLOR)
    }

    /// Gets the `TMT_TRANSPARENTCOLOR` property.
    #[inline]
    pub fn tmt_transparentcolor(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_TRANSPARENTCOLOR)
    }

    /// Gets the `TMT_WINDOW` property.
    #[inline]
    pub fn tmt_window(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_WINDOW)
    }

    /// Gets the `TMT_WINDOWFRAME` property.
    #[inline]
    pub fn tmt_windowframe(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_WINDOWFRAME)
    }

    /// Gets the `TMT_WINDOWTEXT` property.
    #[inline]
    pub fn tmt_windowtext(&self, part: Part) -> crate::GetThemeColorRet {
        self.get_theme_color(part, windows_sys::Win32::UI::Controls::TMT_WINDOWTEXT)
    }

    /// Gets the `TMT_DISKSTREAM` property.
    #[inline]
    pub fn tmt_diskstream(&self, part: Part) -> crate::GetThemeStreamRet {
        self.get_theme_stream(part, windows_sys::Win32::UI::Controls::TMT_DISKSTREAM)
    }

    /// Gets the `TMT_ATLASIMAGE` property.
    #[inline]
    pub fn tmt_atlasimage(&self, part: Part) -> crate::GetThemeStreamRet {
        self.get_theme_stream(part, windows_sys::Win32::UI::Controls::TMT_ATLASIMAGE)
    }

    /// Gets the `TMT_ENUM` property.
    #[inline]
    pub fn tmt_enum(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_ENUM)
    }

    /// Gets the `TMT_BGTYPE` property.
    #[inline]
    pub fn tmt_bgtype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_BGTYPE)
    }

    /// Gets the `TMT_BORDERTYPE` property.
    #[inline]
    pub fn tmt_bordertype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_BORDERTYPE)
    }

    /// Gets the `TMT_CONTENTALIGNMENT` property.
    #[inline]
    pub fn tmt_contentalignment(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_CONTENTALIGNMENT)
    }

    /// Gets the `TMT_FILLTYPE` property.
    #[inline]
    pub fn tmt_filltype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_FILLTYPE)
    }

    /// Gets the `TMT_GLYPHTYPE` property.
    #[inline]
    pub fn tmt_glyphtype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_GLYPHTYPE)
    }

    /// Gets the `TMT_GLYPHFONTSIZINGTYPE` property.
    #[inline]
    pub fn tmt_glyphfontsizingtype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_GLYPHFONTSIZINGTYPE)
    }

    /// Gets the `TMT_HALIGN` property.
    #[inline]
    pub fn tmt_halign(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_HALIGN)
    }

    /// Gets the `TMT_ICONEFFECT` property.
    #[inline]
    pub fn tmt_iconeffect(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_ICONEFFECT)
    }

    /// Gets the `TMT_IMAGELAYOUT` property.
    #[inline]
    pub fn tmt_imagelayout(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_IMAGELAYOUT)
    }

    /// Gets the `TMT_IMAGESELECTTYPE` property.
    #[inline]
    pub fn tmt_imageselecttype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_IMAGESELECTTYPE)
    }

    /// Gets the `TMT_OFFSETTYPE` property.
    #[inline]
    pub fn tmt_offsettype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_OFFSETTYPE)
    }

    /// Gets the `TMT_SIZINGTYPE` property.
    #[inline]
    pub fn tmt_sizingtype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_SIZINGTYPE)
    }

    /// Gets the `TMT_TEXTSHADOWTYPE` property.
    #[inline]
    pub fn tmt_textshadowtype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_TEXTSHADOWTYPE)
    }

    /// Gets the `TMT_TRUESIZESCALINGTYPE` property.
    #[inline]
    pub fn tmt_truesizescalingtype(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_TRUESIZESCALINGTYPE)
    }

    /// Gets the `TMT_VALIGN` property.
    #[inline]
    pub fn tmt_valign(&self, part: Part) -> crate::GetThemeEnumValueRet {
        self.get_theme_enum_value(part, windows_sys::Win32::UI::Controls::TMT_VALIGN)
    }

    /// Gets the `TMT_FILENAME` property.
    #[inline]
    pub fn tmt_filename(&self, part: Part) -> crate::GetThemeFilenameRet {
        self.get_theme_filename(part, windows_sys::Win32::UI::Controls::TMT_FILENAME)
    }

    /// Gets the `TMT_GLYPHIMAGEFILE` property.
    #[inline]
    pub fn tmt_glyphimagefile(&self, part: Part) -> crate::GetThemeFilenameRet {
        self.get_theme_filename(part, windows_sys::Win32::UI::Controls::TMT_GLYPHIMAGEFILE)
    }

    /// Gets the `TMT_IMAGEFILE` property.
    #[inline]
    pub fn tmt_imagefile(&self, part: Part) -> crate::GetThemeFilenameRet {
        self.get_theme_filename(part, windows_sys::Win32::UI::Controls::TMT_IMAGEFILE)
    }

    /// Gets the `TMT_IMAGEFILE1` property.
    #[inline]
    pub fn tmt_imagefile1(&self, part: Part) -> crate::GetThemeFilenameRet {
        self.get_theme_filename(part, windows_sys::Win32::UI::Controls::TMT_IMAGEFILE1)
    }

    /// Gets the `TMT_IMAGEFILE2` property.
    #[inline]
    pub fn tmt_imagefile2(&self, part: Part) -> crate::GetThemeFilenameRet {
        self.get_theme_filename(part, windows_sys::Win32::UI::Controls::TMT_IMAGEFILE2)
    }

    /// Gets the `TMT_IMAGEFILE3` property.
    #[inline]
    pub fn tmt_imagefile3(&self, part: Part) -> crate::GetThemeFilenameRet {
        self.get_theme_filename(part, windows_sys::Win32::UI::Controls::TMT_IMAGEFILE3)
    }

    /// Gets the `TMT_IMAGEFILE4` property.
    #[inline]
    pub fn tmt_imagefile4(&self, part: Part) -> crate::GetThemeFilenameRet {
        self.get_theme_filename(part, windows_sys::Win32::UI::Controls::TMT_IMAGEFILE4)
    }

    /// Gets the `TMT_IMAGEFILE5` property.
    #[inline]
    pub fn tmt_imagefile5(&self, part: Part) -> crate::GetThemeFilenameRet {
        self.get_theme_filename(part, windows_sys::Win32::UI::Controls::TMT_IMAGEFILE5)
    }

    /// Gets the `TMT_FONT` property.
    #[inline]
    pub fn tmt_font(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_FONT)
    }

    /// Gets the `TMT_BODYFONT` property.
    #[inline]
    pub fn tmt_bodyfont(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_BODYFONT)
    }

    /// Gets the `TMT_CAPTIONFONT` property.
    #[inline]
    pub fn tmt_captionfont(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_CAPTIONFONT)
    }

    /// Gets the `TMT_GLYPHFONT` property.
    #[inline]
    pub fn tmt_glyphfont(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_GLYPHFONT)
    }

    /// Gets the `TMT_HEADING1FONT` property.
    #[inline]
    pub fn tmt_heading1font(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_HEADING1FONT)
    }

    /// Gets the `TMT_HEADING2FONT` property.
    #[inline]
    pub fn tmt_heading2font(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_HEADING2FONT)
    }

    /// Gets the `TMT_ICONTITLEFONT` property.
    #[inline]
    pub fn tmt_icontitlefont(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_ICONTITLEFONT)
    }

    /// Gets the `TMT_MENUFONT` property.
    #[inline]
    pub fn tmt_menufont(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_MENUFONT)
    }

    /// Gets the `TMT_MSGBOXFONT` property.
    #[inline]
    pub fn tmt_msgboxfont(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_MSGBOXFONT)
    }

    /// Gets the `TMT_SMALLCAPTIONFONT` property.
    #[inline]
    pub fn tmt_smallcaptionfont(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_SMALLCAPTIONFONT)
    }

    /// Gets the `TMT_STATUSFONT` property.
    #[inline]
    pub fn tmt_statusfont(&self, part: Part) -> crate::GetThemeFontRet {
        self.get_theme_font(part, windows_sys::Win32::UI::Controls::TMT_STATUSFONT)
    }

    /// Gets the `TMT_HBITMAP` property.
    #[inline]
    pub fn tmt_hbitmap(&self, part: Part) -> crate::GetThemeBitmapRet {
        self.get_theme_bitmap(part, windows_sys::Win32::UI::Controls::TMT_HBITMAP)
    }

    /// Gets the `TMT_INT` property.
    #[inline]
    pub fn tmt_int(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_INT)
    }

    /// Gets the `TMT_ALPHALEVEL` property.
    #[inline]
    pub fn tmt_alphalevel(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_ALPHALEVEL)
    }

    /// Gets the `TMT_ALPHATHRESHOLD` property.
    #[inline]
    pub fn tmt_alphathreshold(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_ALPHATHRESHOLD)
    }

    /// Gets the `TMT_ANIMATIONDELAY` property.
    #[inline]
    pub fn tmt_animationdelay(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_ANIMATIONDELAY)
    }

    /// Gets the `TMT_ANIMATIONDURATION` property.
    #[inline]
    pub fn tmt_animationduration(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_ANIMATIONDURATION)
    }

    /// Gets the `TMT_BORDERSIZE` property.
    #[inline]
    pub fn tmt_bordersize(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_BORDERSIZE)
    }

    /// Gets the `TMT_CHARSET` property.
    #[inline]
    pub fn tmt_charset(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_CHARSET)
    }

    /// Gets the `TMT_COLORIZATIONCOLOR` property.
    #[inline]
    pub fn tmt_colorizationcolor(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_COLORIZATIONCOLOR)
    }

    /// Gets the `TMT_COLORIZATIONOPACITY` property.
    #[inline]
    pub fn tmt_colorizationopacity(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_COLORIZATIONOPACITY)
    }

    /// Gets the `TMT_FRAMESPERSECOND` property.
    #[inline]
    pub fn tmt_framespersecond(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_FRAMESPERSECOND)
    }

    /// Gets the `TMT_FROMHUE1` property.
    #[inline]
    pub fn tmt_fromhue1(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_FROMHUE1)
    }

    /// Gets the `TMT_FROMHUE2` property.
    #[inline]
    pub fn tmt_fromhue2(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_FROMHUE2)
    }

    /// Gets the `TMT_FROMHUE3` property.
    #[inline]
    pub fn tmt_fromhue3(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_FROMHUE3)
    }

    /// Gets the `TMT_FROMHUE4` property.
    #[inline]
    pub fn tmt_fromhue4(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_FROMHUE4)
    }

    /// Gets the `TMT_FROMHUE5` property.
    #[inline]
    pub fn tmt_fromhue5(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_FROMHUE5)
    }

    /// Gets the `TMT_GLOWINTENSITY` property.
    #[inline]
    pub fn tmt_glowintensity(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_GLOWINTENSITY)
    }

    /// Gets the `TMT_GLYPHINDEX` property.
    #[inline]
    pub fn tmt_glyphindex(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_GLYPHINDEX)
    }

    /// Gets the `TMT_GRADIENTRATIO1` property.
    #[inline]
    pub fn tmt_gradientratio1(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTRATIO1)
    }

    /// Gets the `TMT_GRADIENTRATIO2` property.
    #[inline]
    pub fn tmt_gradientratio2(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTRATIO2)
    }

    /// Gets the `TMT_GRADIENTRATIO3` property.
    #[inline]
    pub fn tmt_gradientratio3(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTRATIO3)
    }

    /// Gets the `TMT_GRADIENTRATIO4` property.
    #[inline]
    pub fn tmt_gradientratio4(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTRATIO4)
    }

    /// Gets the `TMT_GRADIENTRATIO5` property.
    #[inline]
    pub fn tmt_gradientratio5(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_GRADIENTRATIO5)
    }

    /// Gets the `TMT_HEIGHT` property.
    #[inline]
    pub fn tmt_height(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_HEIGHT)
    }

    /// Gets the `TMT_IMAGECOUNT` property.
    #[inline]
    pub fn tmt_imagecount(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_IMAGECOUNT)
    }

    /// Gets the `TMT_MINCOLORDEPTH` property.
    #[inline]
    pub fn tmt_mincolordepth(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_MINCOLORDEPTH)
    }

    /// Gets the `TMT_MINDPI1` property.
    #[inline]
    pub fn tmt_mindpi1(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_MINDPI1)
    }

    /// Gets the `TMT_MINDPI2` property.
    #[inline]
    pub fn tmt_mindpi2(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_MINDPI2)
    }

    /// Gets the `TMT_MINDPI3` property.
    #[inline]
    pub fn tmt_mindpi3(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_MINDPI3)
    }

    /// Gets the `TMT_MINDPI4` property.
    #[inline]
    pub fn tmt_mindpi4(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_MINDPI4)
    }

    /// Gets the `TMT_MINDPI5` property.
    #[inline]
    pub fn tmt_mindpi5(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_MINDPI5)
    }

    /// Gets the `TMT_OPACITY` property.
    #[inline]
    pub fn tmt_opacity(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_OPACITY)
    }

    /// Gets the `TMT_PIXELSPERFRAME` property.
    #[inline]
    pub fn tmt_pixelsperframe(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_PIXELSPERFRAME)
    }

    /// Gets the `TMT_PROGRESSCHUNKSIZE` property.
    #[inline]
    pub fn tmt_progresschunksize(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_PROGRESSCHUNKSIZE)
    }

    /// Gets the `TMT_PROGRESSSPACESIZE` property.
    #[inline]
    pub fn tmt_progressspacesize(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_PROGRESSSPACESIZE)
    }

    /// Gets the `TMT_ROUNDCORNERHEIGHT` property.
    #[inline]
    pub fn tmt_roundcornerheight(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_ROUNDCORNERHEIGHT)
    }

    /// Gets the `TMT_ROUNDCORNERWIDTH` property.
    #[inline]
    pub fn tmt_roundcornerwidth(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_ROUNDCORNERWIDTH)
    }

    /// Gets the `TMT_SATURATION` property.
    #[inline]
    pub fn tmt_saturation(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_SATURATION)
    }

    /// Gets the `TMT_TEXTBORDERSIZE` property.
    #[inline]
    pub fn tmt_textbordersize(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TEXTBORDERSIZE)
    }

    /// Gets the `TMT_TEXTGLOWSIZE` property.
    #[inline]
    pub fn tmt_textglowsize(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TEXTGLOWSIZE)
    }

    /// Gets the `TMT_TOCOLOR1` property.
    #[inline]
    pub fn tmt_tocolor1(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOCOLOR1)
    }

    /// Gets the `TMT_TOCOLOR2` property.
    #[inline]
    pub fn tmt_tocolor2(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOCOLOR2)
    }

    /// Gets the `TMT_TOCOLOR3` property.
    #[inline]
    pub fn tmt_tocolor3(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOCOLOR3)
    }

    /// Gets the `TMT_TOCOLOR4` property.
    #[inline]
    pub fn tmt_tocolor4(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOCOLOR4)
    }

    /// Gets the `TMT_TOCOLOR5` property.
    #[inline]
    pub fn tmt_tocolor5(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOCOLOR5)
    }

    /// Gets the `TMT_TOHUE1` property.
    #[inline]
    pub fn tmt_tohue1(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOHUE1)
    }

    /// Gets the `TMT_TOHUE2` property.
    #[inline]
    pub fn tmt_tohue2(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOHUE2)
    }

    /// Gets the `TMT_TOHUE3` property.
    #[inline]
    pub fn tmt_tohue3(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOHUE3)
    }

    /// Gets the `TMT_TOHUE4` property.
    #[inline]
    pub fn tmt_tohue4(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOHUE4)
    }

    /// Gets the `TMT_TOHUE5` property.
    #[inline]
    pub fn tmt_tohue5(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TOHUE5)
    }

    /// Gets the `TMT_TRUESIZESTRETCHMARK` property.
    #[inline]
    pub fn tmt_truesizestretchmark(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_TRUESIZESTRETCHMARK)
    }

    /// Gets the `TMT_WIDTH` property.
    #[inline]
    pub fn tmt_width(&self, part: Part) -> crate::GetThemeIntRet {
        self.get_theme_int(part, windows_sys::Win32::UI::Controls::TMT_WIDTH)
    }

    /// Gets the `TMT_INTLIST` property.
    #[inline]
    pub fn tmt_intlist(&self, part: Part) -> crate::GetThemeIntListRet {
        self.get_theme_int_list(part, windows_sys::Win32::UI::Controls::TMT_INTLIST)
    }

    /// Gets the `TMT_TRANSITIONDURATIONS` property.
    #[inline]
    pub fn tmt_transitiondurations(&self, part: Part) -> crate::GetThemeIntListRet {
        self.get_theme_int_list(part, windows_sys::Win32::UI::Controls::TMT_TRANSITIONDURATIONS)
    }

    /// Gets the `TMT_MARGINS` property.
    #[inline]
    pub fn tmt_margins(&self, part: Part) -> crate::GetThemeMarginsRet {
        self.get_theme_margins(part, windows_sys::Win32::UI::Controls::TMT_MARGINS)
    }

    /// Gets the `TMT_CAPTIONMARGINS` property.
    #[inline]
    pub fn tmt_captionmargins(&self, part: Part) -> crate::GetThemeMarginsRet {
        self.get_theme_margins(part, windows_sys::Win32::UI::Controls::TMT_CAPTIONMARGINS)
    }

    /// Gets the `TMT_CONTENTMARGINS` property.
    #[inline]
    pub fn tmt_contentmargins(&self, part: Part) -> crate::GetThemeMarginsRet {
        self.get_theme_margins(part, windows_sys::Win32::UI::Controls::TMT_CONTENTMARGINS)
    }

    /// Gets the `TMT_SIZINGMARGINS` property.
    #[inline]
    pub fn tmt_sizingmargins(&self, part: Part) -> crate::GetThemeMarginsRet {
        self.get_theme_margins(part, windows_sys::Win32::UI::Controls::TMT_SIZINGMARGINS)
    }

    /// Gets the `TMT_POSITION` property.
    #[inline]
    pub fn tmt_position(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_POSITION)
    }

    /// Gets the `TMT_MINSIZE` property.
    #[inline]
    pub fn tmt_minsize(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_MINSIZE)
    }

    /// Gets the `TMT_MINSIZE1` property.
    #[inline]
    pub fn tmt_minsize1(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_MINSIZE1)
    }

    /// Gets the `TMT_MINSIZE2` property.
    #[inline]
    pub fn tmt_minsize2(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_MINSIZE2)
    }

    /// Gets the `TMT_MINSIZE3` property.
    #[inline]
    pub fn tmt_minsize3(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_MINSIZE3)
    }

    /// Gets the `TMT_MINSIZE4` property.
    #[inline]
    pub fn tmt_minsize4(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_MINSIZE4)
    }

    /// Gets the `TMT_MINSIZE5` property.
    #[inline]
    pub fn tmt_minsize5(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_MINSIZE5)
    }

    /// Gets the `TMT_NORMALSIZE` property.
    #[inline]
    pub fn tmt_normalsize(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_NORMALSIZE)
    }

    /// Gets the `TMT_OFFSET` property.
    #[inline]
    pub fn tmt_offset(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_OFFSET)
    }

    /// Gets the `TMT_TEXTSHADOWOFFSET` property.
    #[inline]
    pub fn tmt_textshadowoffset(&self, part: Part) -> crate::GetThemePositionRet {
        self.get_theme_position(part, windows_sys::Win32::UI::Controls::TMT_TEXTSHADOWOFFSET)
    }

    /// Gets the `TMT_RECT` property.
    #[inline]
    pub fn tmt_rect(&self, part: Part) -> crate::GetThemeRectRet {
        self.get_theme_rect(part, windows_sys::Win32::UI::Controls::TMT_RECT)
    }

    /// Gets the `TMT_ANIMATIONBUTTONRECT` property.
    #[inline]
    pub fn tmt_animationbuttonrect(&self, part: Part) -> crate::GetThemeRectRet {
        self.get_theme_rect(part, windows_sys::Win32::UI::Controls::TMT_ANIMATIONBUTTONRECT)
    }

    /// Gets the `TMT_ATLASRECT` property.
    #[inline]
    pub fn tmt_atlasrect(&self, part: Part) -> crate::GetThemeRectRet {
        self.get_theme_rect(part, windows_sys::Win32::UI::Controls::TMT_ATLASRECT)
    }

    /// Gets the `TMT_CUSTOMSPLITRECT` property.
    #[inline]
    pub fn tmt_customsplitrect(&self, part: Part) -> crate::GetThemeRectRet {
        self.get_theme_rect(part, windows_sys::Win32::UI::Controls::TMT_CUSTOMSPLITRECT)
    }

    /// Gets the `TMT_DEFAULTPANESIZE` property.
    #[inline]
    pub fn tmt_defaultpanesize(&self, part: Part) -> crate::GetThemeRectRet {
        self.get_theme_rect(part, windows_sys::Win32::UI::Controls::TMT_DEFAULTPANESIZE)
    }

    /// Gets the `TMT_STRING` property.
    #[inline]
    pub fn tmt_string(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_STRING)
    }

    /// Gets the `TMT_ALIAS` property.
    #[inline]
    pub fn tmt_alias(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_ALIAS)
    }

    /// Gets the `TMT_ATLASINPUTIMAGE` property.
    #[inline]
    pub fn tmt_atlasinputimage(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_ATLASINPUTIMAGE)
    }

    /// Gets the `TMT_AUTHOR` property.
    #[inline]
    pub fn tmt_author(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_AUTHOR)
    }

    /// Gets the `TMT_CLASSICVALUE` property.
    #[inline]
    pub fn tmt_classicvalue(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_CLASSICVALUE)
    }

    /// Gets the `TMT_COLORSCHEMES` property.
    #[inline]
    pub fn tmt_colorschemes(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_COLORSCHEMES)
    }

    /// Gets the `TMT_COMPANY` property.
    #[inline]
    pub fn tmt_company(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_COMPANY)
    }

    /// Gets the `TMT_COPYRIGHT` property.
    #[inline]
    pub fn tmt_copyright(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_COPYRIGHT)
    }

    /// Gets the `TMT_CSSNAME` property.
    #[inline]
    pub fn tmt_cssname(&self) -> crate::GetThemeStringRet {
        self.get_theme_sys_string(windows_sys::Win32::UI::Controls::TMT_CSSNAME)
    }

    /// Gets the `TMT_DESCRIPTION` property.
    #[inline]
    pub fn tmt_description(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_DESCRIPTION)
    }

    /// Gets the `TMT_DISPLAYNAME` property.
    #[inline]
    pub fn tmt_displayname(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_DISPLAYNAME)
    }

    /// Gets the `TMT_LASTUPDATED` property.
    #[inline]
    pub fn tmt_lastupdated(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_LASTUPDATED)
    }

    /// Gets the `TMT_SIZES` property.
    #[inline]
    pub fn tmt_sizes(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_SIZES)
    }

    /// Gets the `TMT_TEXT` property.
    #[inline]
    pub fn tmt_text(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_TEXT)
    }

    /// Gets the `TMT_TOOLTIP` property.
    #[inline]
    pub fn tmt_tooltip(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_TOOLTIP)
    }

    /// Gets the `TMT_URL` property.
    #[inline]
    pub fn tmt_url(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_URL)
    }

    /// Gets the `TMT_VERSION` property.
    #[inline]
    pub fn tmt_version(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_VERSION)
    }

    /// Gets the `TMT_XMLNAME` property.
    #[inline]
    pub fn tmt_xmlname(&self) -> crate::GetThemeStringRet {
        self.get_theme_sys_string(windows_sys::Win32::UI::Controls::TMT_XMLNAME)
    }

    /// Gets the `TMT_NAME` property.
    #[inline]
    pub fn tmt_name(&self, part: Part) -> crate::GetThemeStringRet {
        self.get_theme_string(part, windows_sys::Win32::UI::Controls::TMT_NAME)
    }

}

