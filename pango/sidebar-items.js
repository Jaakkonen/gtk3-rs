initSidebarItems({"constant":[["NONE_ENGINE_LANG",""],["NONE_ENGINE_SHAPE",""],["NONE_FONT",""],["NONE_FONTSET",""],["NONE_FONT_FACE",""],["NONE_FONT_FAMILY",""],["NONE_FONT_MAP",""],["NONE_RENDERER",""],["SCALE",""],["SCALE_LARGE","The scale factor for one magnification step (1.2)."],["SCALE_MEDIUM","The scale factor for normal size (1.0)."],["SCALE_SMALL","The scale factor for one shrinking step (1 / 1.2)."],["SCALE_XX_LARGE","The scale factor for three magnification steps (1.2 * 1.2 * 1.2)."],["SCALE_XX_SMALL","The scale factor for three shrinking steps (1 / (1.2 * 1.2 * 1.2))."],["SCALE_X_LARGE","The scale factor for two magnification steps (1.2 * 1.2)."],["SCALE_X_SMALL","The scale factor for two shrinking steps (1 / (1.2 * 1.2))."]],"enum":[["Alignment","A `Alignment` describes how to align the lines of a `Layout` within the available space. If the `Layout` is set to justify using `Layout::set_justify`, this only has effect for partial lines."],["AttrType","The `AttrType` distinguishes between different types of attributes. Along with the predefined values, it is possible to allocate additional values for custom attributes using `AttrType::register`. The predefined values are given below. The type of structure used to store the attribute is listed in parentheses after the description."],["BidiType","The `BidiType` type represents the bidirectional character type of a Unicode character as specified by the Unicode bidirectional algorithm`</ulink>`."],["CoverageLevel","Used to indicate how well a font can represent a particular Unicode character point for a particular script."],["Direction","The `Direction` type represents a direction in the Unicode bidirectional algorithm; not every value in this enumeration makes sense for every usage of `Direction`; for example, the return value of `pango_unichar_direction` and `pango_find_base_dir` cannot be `Direction::WeakLtr` or `Direction::WeakRtl`, since every character is either neutral or has a strong direction; on the other hand `Direction::Neutral` doesn’t make sense to pass to `pango_itemize_with_base_dir`."],["EllipsizeMode","The `EllipsizeMode` type describes what sort of (if any) ellipsization should be applied to a line of text. In the ellipsization process characters are removed from the text in order to make it fit to a given width and replaced with an ellipsis."],["Gravity","The `Gravity` type represents the orientation of glyphs in a segment of text. This is useful when rendering vertical text layouts. In those situations, the layout is rotated using a non-identity PangoMatrix, and then glyph orientation is controlled using `Gravity`. Not every value in this enumeration makes sense for every usage of `Gravity`; for example, `Gravity::Auto` only can be passed to `Context::set_base_gravity` and can only be returned by `Context::get_base_gravity`."],["GravityHint","The `GravityHint` defines how horizontal scripts should behave in a vertical context. That is, English excerpt in a vertical paragraph for example."],["Overline",""],["RenderPart","`RenderPart` defines different items to render for such purposes as setting colors."],["Script","The `Script` enumeration identifies different writing systems. The values correspond to the names as defined in the Unicode standard. See Unicode Standard Annex `24`: Script names`</ulink>`."],["Stretch","An enumeration specifying the width of the font relative to other designs within a family."],["Style","An enumeration specifying the various slant styles possible for a font."],["TabAlign","A `TabAlign` specifies where a tab stop appears relative to the text."],["Underline","The `Underline` enumeration is used to specify whether text should be underlined, and if so, the type of underlining."],["Variant","An enumeration specifying capitalization variant of the font."],["Weight","An enumeration specifying the weight (boldness) of a font. This is a numerical value ranging from 100 to 1000, but there are some predefined values:"],["WrapMode","A `WrapMode` describes how to wrap the lines of a `Layout` to the desired width."]],"fn":[["extents_to_pixels",""],["find_base_dir",""],["find_paragraph_boundary",""],["is_zero_width",""],["itemize",""],["itemize_with_base_dir",""],["parse_enum",""],["parse_markup",""],["parse_stretch",""],["parse_style",""],["parse_variant",""],["parse_weight",""],["quantize_line_geometry",""],["reorder_items",""],["shape",""],["shape_full",""],["shape_with_flags",""],["split_file_list",""],["trim_string",""],["unichar_direction",""],["units_from_double",""],["units_to_double",""],["version",""],["version_check",""],["version_string",""]],"mod":[["analysis",""],["attr_class",""],["attr_iterator",""],["attr_list",""],["attribute",""],["functions",""],["glyph",""],["item",""],["language",""],["layout",""],["prelude","Traits and essential types inteded for blanket imports."],["rectangle",""]],"static":[["ENGINE_TYPE_LANG",""],["ENGINE_TYPE_SHAPE",""],["RENDER_TYPE_NONE",""]],"struct":[["AttrIterator","The `AttrIterator` structure is used to represent an iterator through a `AttrList`. A new iterator is created with `AttrList::get_iterator`. Once the iterator is created, it can be advanced through the style changes in the text using `AttrIterator::next`. At each style change, the range of the current style segment and the attributes currently in effect can be queried."],["AttrList","The `AttrList` structure represents a list of attributes that apply to a section of text. The attributes are, in general, allowed to overlap in an arbitrary fashion, however, if the attributes are manipulated only through `AttrList::change`, the overlap between properties will meet stricter criteria."],["Attribute","The `Attribute` structure represents the common portions of all attributes. Particular types of attributes include this structure as their initial portion. The common portion of the attribute holds the range to which the value in the type-specific part of the attribute applies and should be initialized using `Attribute::init`. By default an attribute will have an all-inclusive range of [0,`G_MAXUINT`]."],["Color","The `Color` structure is used to represent a color in an uncalibrated RGB color-space."],["Context","The `Context` structure stores global information used to control the itemization process."],["Coverage","The `Coverage` structure represents a map from Unicode characters to `CoverageLevel`. It is an opaque structure with no public fields."],["EngineLang","`[Deprecated since 1.38]` The `EngineLang` class is implemented by engines that customize the rendering-system independent part of the Pango pipeline for a particular script or language. For instance, a custom `EngineLang` could be provided for Thai to implement the dictionary-based word boundary lookups needed for that language."],["EngineShape","`[Deprecated since 1.38]` The `EngineShape` class is implemented by engines that customize the rendering-system dependent part of the Pango pipeline for a particular script or language. A `EngineShape` implementation is then specific to both a particular rendering system or group of rendering systems and to a particular script. For instance, there is one `EngineShape` implementation to handle shaping Arabic for Fontconfig-based backends."],["Font","The `Font` structure is used to represent a font in a rendering-system-independent matter. To create an implementation of a `Font`, the rendering-system specific code should allocate a larger structure that contains a nested `Font`, fill in the `<structfield>`klass`</structfield>` member of the nested `Font` with a pointer to a appropriate `FontClass`, then call `pango_font_init` on the structure."],["FontDescription","The `FontDescription` structure represents the description of an ideal font. These structures are used both to list what fonts are available on the system and also for specifying the characteristics of a font to load."],["FontFace","The `FontFace` structure is used to represent a group of fonts with the same family, slant, weight, width, but varying sizes."],["FontFamily","The `FontFamily` structure is used to represent a family of related font faces. The faces in a family share a common design, but differ in slant, weight, width and other aspects."],["FontMap","The `FontMap` represents the set of fonts available for a particular rendering system. This is a virtual object with implementations being specific to particular rendering systems. To create an implementation of a `FontMap`, the rendering-system specific code should allocate a larger structure that contains a nested `FontMap`, fill in the `<structfield>`klass`</structfield>` member of the nested `FontMap` with a pointer to a appropriate `FontMapClass`, then call `pango_font_map_init` on the structure."],["FontMask","The bits in a `FontMask` correspond to fields in a `FontDescription` that have been set."],["FontMetrics","A `FontMetrics` structure holds the overall metric information for a font (possibly restricted to a script). The fields of this structure are private to implementations of a font backend. See the documentation of the corresponding getters for documentation of their meaning."],["Fontset","A `Fontset` represents a set of `Font` to use when rendering text. It is the result of resolving a `FontDescription` against a particular `Context`. It has operations for finding the component font for a particular Unicode character, and for finding a composite set of metrics for the entire fontset."],["FontsetSimple","`FontsetSimple` is a implementation of the abstract `Fontset` base class in terms of an array of fonts, which the creator provides when constructing the `FontsetSimple`."],["GlyphItem","A `GlyphItem` is a pair of a `Item` and the glyphs resulting from shaping the text corresponding to an item. As an example of the usage of `GlyphItem`, the results of shaping text with `Layout` is a list of `LayoutLine`, each of which contains a list of `GlyphItem`."],["GlyphItemIter","A `GlyphItemIter` is an iterator over the clusters in a `GlyphItem`. The `<firstterm>`forward direction`</firstterm>` of the iterator is the logical direction of text. That is, with increasing `start_index` and `start_char` values. If `glyph_item` is right-to-left (that is, if `<literal>``glyph_item`->item->analysis.level`</literal>` is odd), then `start_glyph` decreases as the iterator moves forward. Moreover, in right-to-left cases, `start_glyph` is greater than `end_glyph`."],["GlyphString","The `GlyphString` structure is used to store strings of glyphs with geometry and visual attribute information. The storage for the glyph information is owned by the structure which simplifies memory management."],["Item","The `Item` structure stores information about a segment of text."],["Layout","The `Layout` structure represents an entire paragraph of text. It is initialized with a `Context`, UTF-8 string and set of attributes for that string. Once that is done, the set of formatted lines can be extracted from the object, the layout can be rendered, and conversion between logical character positions within the layout’s text, and the physical position of the resulting glyphs can be made."],["LayoutIter","A `LayoutIter` structure can be used to iterate over the visual extents of a `Layout`."],["LayoutLine","The `LayoutLine` structure represents one of the lines resulting from laying out a paragraph via `Layout`. `LayoutLine` structures are obtained by calling `Layout::get_line` and are only valid until the text, attributes, or settings of the parent `Layout` are modified."],["Matrix","A structure specifying a transformation between user-space coordinates and device coordinates. The transformation is given by"],["Renderer","`Renderer` is a base class for objects that are used to render Pango objects such as `GlyphString` and `Layout`."],["ShapeFlags",""],["ShowFlags",""],["TabArray","A `TabArray` struct contains an array of tab stops. Each tab stop has an alignment and a position."]],"trait":[["FontExt","Trait containing all `Font` methods."],["FontFaceExt","Trait containing all `FontFace` methods."],["FontFamilyExt","Trait containing all `FontFamily` methods."],["FontMapExt","Trait containing all `FontMap` methods."],["FontsetExt","Trait containing all `Fontset` methods."],["RendererExt","Trait containing all `Renderer` methods."]],"type":[["Glyph",""],["GlyphUnit",""],["LayoutRun",""]]});