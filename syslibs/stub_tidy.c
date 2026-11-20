void tidyCreate() {}
void tidyRelease() {}
void tidyBufInit() {}
void tidyBufFree() {}
void tidyBufAttach() {}
void tidyBufDetach() {}
int tidySetErrorBuffer() { return 0; }
int tidyOptSetBool() { return 0; }
int tidyOptSetInt() { return 0; }
int tidyOptSetValue() { return 0; }
int tidyOptGetBool() { return 0; }
unsigned long tidyOptGetInt() { return 0; }
const char* tidyOptGetValue() { return ""; }
void* tidyGetOptionByName() { return 0; }
int tidyOptGetId() { return 0; }
int tidyOptGetIdForName() { return 0; }
const char* tidyOptGetName() { return ""; }
int tidyOptGetType() { return 0; }
int tidyOptGetCategory() { return 0; }
const char* tidyOptGetDoc() { return ""; }
void* tidyGetOptionList() { return 0; }
void* tidyGetNextOption() { return 0; }
int tidyLoadConfig() { return 0; }
int tidySetCharEncoding() { return 0; }
int tidyParseBuffer() { return 0; }
int tidyCleanAndRepair() { return 0; }
int tidyRunDiagnostics() { return 0; }
int tidySaveBuffer() { return 0; }
int tidyStatus() { return 0; }
int tidyDetectedHtmlVersion() { return 0; }
int tidyDetectedXhtml() { return 0; }
int tidyDetectedGenericXml() { return 0; }
int tidyErrorCount() { return 0; }
int tidyWarningCount() { return 0; }
int tidyAccessWarningCount() { return 0; }
int tidyConfigErrorCount() { return 0; }
void* tidyGetRoot() { return 0; }
void* tidyGetHtml() { return 0; }
void* tidyGetHead() { return 0; }
void* tidyGetBody() { return 0; }
void* tidyGetParent() { return 0; }
void* tidyGetPrev() { return 0; }
void* tidyGetNext() { return 0; }
void* tidyGetChild() { return 0; }
void tidyNodeGetText() {}
const char* tidyNodeGetName() { return ""; }
int tidyNodeGetType() { return 0; }
int tidyNodeLine() { return 0; }
int tidyNodeColumn() { return 0; }
int tidyNodeIsProp() { return 0; }
int tidyNodeGetId() { return 0; }
void* tidyAttrFirst() { return 0; }
const char* tidyAttrName() { return ""; }
const char* tidyAttrValue() { return ""; }
void* tidyAttrNext() { return 0; }
const char* tidyReleaseDate() { return "unknown"; }
const char* tidyLibraryVersion() { return "5.8.0"; }
