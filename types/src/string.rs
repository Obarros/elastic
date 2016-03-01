//! Implementation of the Elasticsearch `string` type.
//! 
//! Strings are stored as a sequence of tokens, constructed based on the given `analyzer`.
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/string.html)

use std::collections::BTreeMap;
use std::marker::PhantomData;
use ::mapping::{ ElasticMapping, ElasticType };

/// The base requirements for mapping a `string` type.
/// 
/// # Examples
/// 
/// Custom mappings can be defined by implementing `ElasticStringMapping`:
/// 
/// ```
/// use std::collections::BTreeMap;
/// use elastic_types::string::{ ElasticStringMapping, ElasticStringFieldMapping, NullValue };
/// 
/// struct MyStringMapping;
/// impl ElasticStringMapping for MyStringMapping {
/// 	fn get_null_value() -> Option<NullValue> {
/// 		Some(NullValue::Default("my default value"))
/// 	}
/// 
/// 	fn get_fields() -> Option<BTreeMap<&'static str, ElasticStringFieldMapping>> {
/// 		let mut fields = BTreeMap::new();
/// 		fields.insert("raw", ElasticStringFieldMapping {
/// 			analyzer: Some("my_analyzer"),
/// 			..Default::default()
/// 		});
/// 		fields.insert("bm25_field", ElasticStringFieldMapping {
/// 			similarity: Some("BM25"),
/// 			..Default::default()
/// 		});
/// 		
/// 		Some(fields)
/// 	}
/// }
/// ```
pub trait ElasticStringMapping {
	/// The analyzer which should be used for analyzed string fields, 
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`). 
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	fn get_analyzer() -> Option<&'static str> {
		None
	}

	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting? 
	/// Accepts disabled or `paged_bytes` (default). 
	/// Not analyzed fields will use doc values in preference to fielddata.
	fn get_fielddata() -> Option<FieldData> {
		None
	}

	/// Multi-fields allow the same string value to be indexed in multiple ways for different purposes, 
	/// such as one field for search and a multi-field for sorting and aggregations, 
	/// or the same string value analyzed by different analyzers.
	fn get_fields() -> Option<BTreeMap<&'static str, ElasticStringFieldMapping>> {
		None
	}

	/// Do not index or analyze any string longer than this value. Defaults to 0 (disabled).
	fn get_ignore_above() -> Option<usize> {
		None
	}

	/// What information should be stored in the index, for search and highlighting purposes. 
	/// Defaults to positions for analyzed fields, and to docs for not_analyzed fields.
	fn get_index_options() -> Option<IndexOptions> {
		None
	}

	/// Whether field-length should be taken into account when scoring queries.
	fn get_norms() -> Option<Norms> {
		None
	}

	/// Accepts a string value which is substituted for any explicit null values. 
	/// Defaults to null, which means the field is treated as missing.
	fn get_null_value() -> Option<NullValue> {
		None
	}

	/// Whether field-length should be taken into account when scoring queries. 
	fn get_position_increment_gap() -> Option<usize> {
		None
	}

	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	fn get_search_analyzer() -> Option<&'static str> {
		None
	}

	/// The analyzer that should be used at search time when a phrase is encountered. 
	/// Defaults to the search_analyzer setting.
	fn get_search_quote_analyzer() -> Option<&'static str> {
		None
	}

	/// Which scoring algorithm or similarity should be used. Defaults to `default`, which uses TF/IDF.
	fn get_similarity() -> Option<&'static str> {
		None
	}

	/// Whether term vectors should be stored for an analyzed field. Defaults to `no`.
	fn get_term_vector() -> Option<TermVector> {
		None
	}
}

/// A multi-field string mapping.
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticStringFieldMapping {
/// The analyzer which should be used for analyzed string fields, 
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`). 
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	pub analyzer: Option<&'static str>,
	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting? 
	/// Accepts disabled or `paged_bytes` (default). 
	/// Not analyzed fields will use doc values in preference to fielddata.
	pub fielddata: Option<FieldData>,
	/// Do not index or analyze any string longer than this value. Defaults to 0 (disabled).
	pub ignore_above: Option<usize>,
	/// What information should be stored in the index, for search and highlighting purposes. 
	/// Defaults to positions for analyzed fields, and to docs for not_analyzed fields.
	pub index_options: Option<IndexOptions>,
	/// Whether field-length should be taken into account when scoring queries.
	pub norms: Option<Norms>,
	/// Accepts a string value which is substituted for any explicit null values. 
	/// Defaults to null, which means the field is treated as missing.
	pub null_value: Option<NullValue>,
	/// Whether field-length should be taken into account when scoring queries. 
	pub position_increment_gap: Option<usize>,
	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	pub search_analyzer: Option<&'static str>,
	/// The analyzer that should be used at search time when a phrase is encountered. 
	/// Defaults to the search_analyzer setting.
	pub search_quote_analyzer: Option<&'static str>,
	/// Which scoring algorithm or similarity should be used. Defaults to `default`, which uses TF/IDF.
	pub similarity: Option<&'static str>,
	/// Whether term vectors should be stored for an analyzed field. Defaults to `no`.
	pub term_vector: Option<TermVector>
}

/// Can the field use in memory fielddata for sorting, aggregations, or scripting?
#[derive(Debug, Clone, Copy)]
pub enum FieldData {
	/// Allow in-memory fielddata.
	PagedBytes(Option<FieldDataLoading>, Option<FieldDataFilter>),
	/// Disallow in-memory fielddata.
	Disabled
}
impl FieldData {
	/// Parse field data from in input string.
	/// 
	/// This will only look at the name, not other details
	pub fn parse(fd: &str) -> FieldData {
		match fd {
			"disabled" => FieldData::Disabled,
			_ => FieldData::default()
		}
	}
}

impl Default for FieldData {
	fn default() -> FieldData {
		FieldData::PagedBytes(None, None)
	}
}

/// This per-field setting controls when fielddata is loaded into memory.
#[derive(Debug, Clone, Copy)]
pub enum FieldDataLoading {
	/// Fielddata is only loaded into memory when it is needed. (default).
	Lazy,
	/// Fielddata is loaded into memory before a new search segment becomes visible to search. 
	/// This can reduce the latency that a user may experience if their search request has to 
	/// trigger lazy loading from a big segment.
	Eager,
	/// Loading fielddata into memory is only part of the work that is required. 
	/// After loading the fielddata for each segment, Elasticsearch builds the 
	/// Global ordinals data structure to make a list of all unique terms across all the segments in a shard. 
	/// By default, global ordinals are built lazily. If the field has a very high cardinality, 
	/// global ordinals may take some time to build, in which case you can use eager loading instead.
	EagerGlobalOrdinals
}

impl Default for FieldDataLoading {
	fn default() -> FieldDataLoading {
		FieldDataLoading::Lazy
	}
}

/// Fielddata filtering can be used to reduce the number of terms loaded into memory, and thus reduce memory usage.
#[derive(Debug, Clone, Copy)]
pub enum FieldDataFilter {
	/// The frequency filter allows you to only load terms whose term frequency falls between a min and max value, 
	/// which can be expressed an absolute number (when the number is bigger than 1.0) or as a percentage (eg 0.01 is 1% and 1.0 is 100%). 
	/// Frequency is calculated per segment. Percentages are based on the number of docs which have a value for the field, 
	/// as opposed to all docs in the segment.
	Frequency { 
		/// The min frequency.
		min: f32, 
		/// The max frequency.
		max: f32, 
		/// The minimum segment size before loading.
		min_segment_size: usize 
	},
	/// Terms can also be filtered by regular expression - only values which match the regular expression are loaded. 
	/// Note: the regular expression is applied to each term in the field, not to the whole field value. 
	Regex { 
		/// The regex pattern.
		pattern: &'static str 
	}
}

/// The index_options parameter controls what information is added to the inverted index, for search and highlighting purposes.
#[derive(Debug, Clone, Copy)]
pub enum IndexOptions {
	/// Only the doc number is indexed. Can answer the question Does this term exist in this field?
	Docs,
	/// Doc number and term frequencies are indexed. 
	/// Term frequencies are used to score repeated terms higher than single terms.
	Freqs,
	/// Doc number, term frequencies, and term positions (or order) are indexed. 
	/// Positions can be used for proximity or phrase queries.
	Positions,
	/// Doc number, term frequencies, positions, 
	/// and start and end character offsets (which map the term back to the original string) are indexed. 
	/// Offsets are used by the postings highlighter.
	Offsets
}

/// Accepts a date value in one of the configured format's as the field which is substituted for any explicit null values. 
/// Defaults to `null`, which means the field is treated as missing.
#[derive(Debug, Clone, Copy)]
pub enum NullValue {
	/// Don't substitute missing fields.
	Null,
	/// Substitute missing fields with a default value.
	Default(&'static str)
}

/// Whether field-length should be taken into account when scoring queries. 
#[derive(Debug, Clone, Copy)]
pub enum Norms {
	/// Enabled norms with eagerness.
	Enabled { 
		/// Whether the loading is eager or lazy.
		loading: NormsLoading 
	},
	/// Disabled norms.
	Disabled
}

/// Whether the norms should be loaded into memory eagerly (`eager`), 
/// whenever a new segment comes online, or they can loaded lazily (`lazy`, default).
#[derive(Debug, Clone, Copy)]
pub enum NormsLoading {
	/// Load norms whenever a new segment comes online.
	Eager,
	/// Load norms lazily.
	Lazy
}

/// Term vectors contain information about the terms produced by the analysis process.
#[derive(Debug, Clone, Copy)]
pub enum TermVector {
	/// No term vectors are stored. (default)
	No,
	/// Just the terms in the field are stored.
	Yes,
	/// Terms and positions are stored.
	WithPositions,
	/// Terms and character offsets are stored.
	WithOffsets,
	/// Terms, positions, and character offsets are stored.
	WithPositionsOffsets
}

/// Default mapping for `String`.
pub struct DefaultStringMapping;
impl ElasticStringMapping for DefaultStringMapping {

}

impl ElasticMapping for DefaultStringMapping {

}

/// A Rust representation of an Elasticsearch `string`.
pub trait ElasticStringType<T: ElasticMapping + ElasticStringMapping> where Self: Sized + ElasticType<T> {

}

impl ElasticType<DefaultStringMapping> for String {

}

impl ElasticStringType<DefaultStringMapping> for String {

}

/// An Elasticsearch `string` with a mapping.
/// 
/// Where the mapping isn't custom, you can use the standard library `String` instead.
/// 
/// # Examples
/// Defining a string with a format:
/// 
/// ```
/// use elastic_types::string::{ ElasticString, DefaultStringMapping };
/// 
/// let string = ElasticString::<DefaultStringMapping>::new("my string value");
/// ```
pub struct ElasticString<T: ElasticMapping + ElasticStringMapping> {
	value: String,
	phantom: PhantomData<T>
}
impl <T: ElasticMapping + ElasticStringMapping> ElasticString<T> {
	/// Creates a new `ElasticString` with the given mapping.
	/// 
	/// # Examples
	/// 
	/// Create a new `ElasticString` from a `String`:
	/// 
	/// ```
	/// use elastic_types::string::{ ElasticString, DefaultStringMapping };
	/// 
	/// let string = ElasticString::<DefaultStringMapping>::new(String::from("my string"));
	/// ```
	pub fn new<I: Into<String>>(string: I) -> ElasticString<T> {
		ElasticString {
			value: string.into(),
			phantom: PhantomData
		}
	}

	/// Get the value of the string.
	pub fn get<'a>(&'a self) -> &'a str {
		&self.value
	}

	/// Set the value of the string.
	pub fn set<I: Into<String>>(&mut self, string: I) {
		self.value = string.into()
	}

	/// Change the mapping of this string.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use elastic_types::ElasticMapping;
	/// use elastic_types::string::{ ElasticString, ElasticStringMapping, DefaultStringMapping };
	/// 
	/// //Define a custom mapping that adjusts the boost value
	/// struct MyStringMapping;
	/// impl ElasticStringMapping for MyStringMapping { }
	/// impl ElasticMapping for MyStringMapping {
	/// 	fn get_boost() -> Option<f32> {
	/// 		Some(2.0)
	/// 	}
	/// }
	/// 
	/// //Use the default format, where boost is given no value.
	/// let string1 = ElasticString::<DefaultStringMapping>::new("my string");
	/// 
	/// let string2: ElasticString<MyStringMapping> = string1.into();
	/// ```
	pub fn into<TInto: ElasticMapping + ElasticStringMapping>(self) -> ElasticString<TInto> {
		ElasticString::<TInto>::new(self.value)
	}
}

impl <T: ElasticMapping + ElasticStringMapping> ElasticType<T> for ElasticString<T> {

}

impl <T: ElasticMapping + ElasticStringMapping> ElasticStringType<T> for ElasticString<T> {

}

impl From<String> for ElasticString<DefaultStringMapping> {
	fn from(string: String) -> Self {
		ElasticString::new(string)
	}
}

impl <T: ElasticMapping + ElasticStringMapping> Into<String> for ElasticString<T> {
	fn into(self) -> String {
		self.value
	}
}