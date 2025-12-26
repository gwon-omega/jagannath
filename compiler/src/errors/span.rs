//! Source Span and Location (Sthāna - स्थान)
//!
//! Tracks source code locations for error reporting.

use std::fmt;
use std::ops::Range;

/// Source file identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceId(pub u32);

impl SourceId {
    /// Create a new source ID
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

/// A span in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Source file
    pub source: SourceId,
    /// Start byte offset
    pub start: u32,
    /// End byte offset
    pub end: u32,
}

impl Span {
    /// Create a new span
    pub fn new(source: SourceId, start: u32, end: u32) -> Self {
        Self { source, start, end }
    }

    /// Create a dummy span for generated code
    pub fn dummy() -> Self {
        Self {
            source: SourceId(0),
            start: 0,
            end: 0,
        }
    }

    /// Combine two spans
    pub fn merge(self, other: Span) -> Span {
        debug_assert_eq!(self.source, other.source);
        Span {
            source: self.source,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Get the byte range
    pub fn range(&self) -> Range<usize> {
        self.start as usize..self.end as usize
    }

    /// Check if this span is before another
    pub fn is_before(&self, other: &Span) -> bool {
        self.end <= other.start
    }

    /// Check if this span contains another
    pub fn contains(&self, other: &Span) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Detailed source location with line/column
#[derive(Debug, Clone, Copy)]
pub struct Location {
    /// Source file
    pub source: SourceId,
    /// Line number (1-indexed)
    pub line: u32,
    /// Column number (1-indexed)
    pub column: u32,
    /// Byte offset
    pub offset: u32,
}

impl Location {
    /// Create a new location
    pub fn new(source: SourceId, line: u32, column: u32, offset: u32) -> Self {
        Self { source, line, column, offset }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Source map for converting byte offsets to line/column
#[derive(Debug, Clone)]
pub struct SourceMap {
    /// File path
    pub path: String,
    /// Source content
    pub content: String,
    /// Line start offsets
    pub lines: Vec<u32>,
}

impl SourceMap {
    /// Create a new source map
    pub fn new(path: String, content: String) -> Self {
        let mut lines = vec![0];
        for (i, c) in content.char_indices() {
            if c == '\n' {
                lines.push(i as u32 + 1);
            }
        }
        Self { path, content, lines }
    }

    /// Get the location for a byte offset
    pub fn location(&self, source: SourceId, offset: u32) -> Location {
        let line = self.lines.partition_point(|&start| start <= offset) as u32;
        let line_start = self.lines[(line - 1) as usize];
        let column = offset - line_start + 1;
        Location::new(source, line, column, offset)
    }

    /// Get a line of source code
    pub fn line(&self, line_num: u32) -> &str {
        let idx = (line_num - 1) as usize;
        if idx >= self.lines.len() {
            return "";
        }
        let start = self.lines[idx] as usize;
        let end = self.lines.get(idx + 1)
            .map(|&e| e as usize - 1)  // Exclude newline
            .unwrap_or(self.content.len());
        &self.content[start..end]
    }

    /// Get source snippet around a span
    pub fn snippet(&self, span: &Span) -> &str {
        let start = span.start as usize;
        let end = span.end as usize;
        &self.content[start.min(self.content.len())..end.min(self.content.len())]
    }
}

/// Collection of source maps
#[derive(Debug, Default)]
pub struct SourceCache {
    sources: Vec<SourceMap>,
}

impl SourceCache {
    /// Create a new cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a source file
    pub fn add(&mut self, path: String, content: String) -> SourceId {
        let id = SourceId(self.sources.len() as u32);
        self.sources.push(SourceMap::new(path, content));
        id
    }

    /// Get a source map by ID
    pub fn get(&self, id: SourceId) -> Option<&SourceMap> {
        self.sources.get(id.0 as usize)
    }

    /// Get location for a span's start
    pub fn location(&self, span: &Span) -> Option<Location> {
        self.get(span.source).map(|sm| sm.location(span.source, span.start))
    }
}
