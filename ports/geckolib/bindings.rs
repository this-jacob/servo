/* automatically generated by rust-bindgen */

pub enum nsIAtom { }
pub enum nsINode { }
pub type RawGeckoNode = nsINode;
pub enum Element { }
pub type RawGeckoElement = Element;
pub enum nsIDocument { }
pub type RawGeckoDocument = nsIDocument;
pub enum ServoNodeData { }
pub enum ServoComputedValues { }
pub enum RawServoStyleSheet { }
pub enum RawServoStyleSet { }
extern "C" {
    pub fn Gecko_ChildrenCount(node: *mut RawGeckoNode) -> u32;
    pub fn Gecko_NodeIsElement(node: *mut RawGeckoNode) -> bool;
    pub fn Gecko_GetParentNode(node: *mut RawGeckoNode) -> *mut RawGeckoNode;
    pub fn Gecko_GetFirstChild(node: *mut RawGeckoNode) -> *mut RawGeckoNode;
    pub fn Gecko_GetLastChild(node: *mut RawGeckoNode) -> *mut RawGeckoNode;
    pub fn Gecko_GetPrevSibling(node: *mut RawGeckoNode) -> *mut RawGeckoNode;
    pub fn Gecko_GetNextSibling(node: *mut RawGeckoNode) -> *mut RawGeckoNode;
    pub fn Gecko_GetParentElement(element: *mut RawGeckoElement)
     -> *mut RawGeckoElement;
    pub fn Gecko_GetFirstChildElement(element: *mut RawGeckoElement)
     -> *mut RawGeckoElement;
    pub fn Gecko_GetLastChildElement(element: *mut RawGeckoElement)
     -> *mut RawGeckoElement;
    pub fn Gecko_GetPrevSiblingElement(element: *mut RawGeckoElement)
     -> *mut RawGeckoElement;
    pub fn Gecko_GetNextSiblingElement(element: *mut RawGeckoElement)
     -> *mut RawGeckoElement;
    pub fn Gecko_GetDocumentElement(document: *mut RawGeckoDocument)
     -> *mut RawGeckoElement;
    pub fn Gecko_ElementState(element: *mut RawGeckoElement) -> u8;
    pub fn Gecko_IsHTMLElementInHTMLDocument(element: *mut RawGeckoElement)
     -> bool;
    pub fn Gecko_IsLink(element: *mut RawGeckoElement) -> bool;
    pub fn Gecko_IsTextNode(node: *mut RawGeckoNode) -> bool;
    pub fn Gecko_IsVisitedLink(element: *mut RawGeckoElement) -> bool;
    pub fn Gecko_IsUnvisitedLink(element: *mut RawGeckoElement) -> bool;
    pub fn Gecko_IsRootElement(element: *mut RawGeckoElement) -> bool;
    pub fn Gecko_GetNodeData(node: *mut RawGeckoNode) -> *mut ServoNodeData;
    pub fn Gecko_SetNodeData(node: *mut RawGeckoNode,
                             data: *mut ServoNodeData);
    pub fn Servo_DropNodeData(data: *mut ServoNodeData);
    pub fn Servo_StylesheetFromUTF8Bytes(bytes: *const u8, length: u32)
     -> *mut RawServoStyleSheet;
    pub fn Servo_AddRefStyleSheet(sheet: *mut RawServoStyleSheet);
    pub fn Servo_ReleaseStyleSheet(sheet: *mut RawServoStyleSheet);
    pub fn Servo_AppendStyleSheet(sheet: *mut RawServoStyleSheet,
                                  set: *mut RawServoStyleSet);
    pub fn Servo_PrependStyleSheet(sheet: *mut RawServoStyleSheet,
                                   set: *mut RawServoStyleSet);
    pub fn Servo_RemoveStyleSheet(sheet: *mut RawServoStyleSheet,
                                  set: *mut RawServoStyleSet);
    pub fn Servo_StyleSheetHasRules(sheet: *mut RawServoStyleSheet) -> bool;
    pub fn Servo_InitStyleSet() -> *mut RawServoStyleSet;
    pub fn Servo_DropStyleSet(set: *mut RawServoStyleSet);
    pub fn Gecko_GetAttrAsUTF8(element: *mut RawGeckoElement, ns: *const u8,
                               name: *const u8, length: *mut u32)
     -> *const ::std::os::raw::c_char;
    pub fn Gecko_LocalName(element: *mut RawGeckoElement, length: *mut u32)
     -> *const u16;
    pub fn Gecko_Namespace(element: *mut RawGeckoElement, length: *mut u32)
     -> *const u16;
    pub fn Servo_GetComputedValues(element: *mut RawGeckoElement)
     -> *mut ServoComputedValues;
    pub fn Servo_GetComputedValuesForAnonymousBox(parentStyleOrNull:
                                                      *mut ServoComputedValues,
                                                  pseudoTag: *mut nsIAtom)
     -> *mut ServoComputedValues;
    pub fn Servo_AddRefComputedValues(arg1: *mut ServoComputedValues);
    pub fn Servo_ReleaseComputedValues(arg1: *mut ServoComputedValues);
    pub fn Servo_RestyleDocument(doc: *mut RawGeckoDocument,
                                 set: *mut RawServoStyleSet);
}
