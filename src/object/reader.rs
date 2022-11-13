// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Result;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::AsyncRead;
use time::OffsetDateTime;

use crate::BytesReader;
use crate::ObjectMetadata;
use crate::ObjectMode;

/// ObjectReader is a bytes reader that carries it's related metadata.
/// Users could fetch part of metadata that carried by read response.
pub struct ObjectReader {
    inner: BytesReader,
    meta: ObjectMetadata,
}

impl ObjectReader {
    /// Create a new object reader.
    pub fn new(inner: BytesReader) -> Self {
        ObjectReader {
            inner,
            // the object meta must be file.
            meta: ObjectMetadata::new(ObjectMode::FILE),
        }
    }

    /// Update object reader's meta with given one.
    pub fn with_meta(mut self, meta: ObjectMetadata) -> Self {
        self.meta = meta;
        self
    }

    /// Content length of this object reader.
    ///
    /// `Content-Length` is defined by [RFC 7230](https://httpwg.org/specs/rfc7230.html#header.content-length)
    /// Refer to [MDN Content-Length](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Length) for more information.
    ///
    /// # Notes
    ///
    /// The content length returned here is the length of this read request.
    /// It's **different** from the object's content length.
    pub fn content_length(&self) -> Option<u64> {
        self.meta.content_length_raw()
    }

    /// Last modified of this object.
    ///
    /// `Last-Modified` is defined by [RFC 7232](https://httpwg.org/specs/rfc7232.html#header.last-modified)
    /// Refer to [MDN Last-Modified](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Last-Modified) for more information.
    ///
    /// OpenDAL parse the raw value into [`OffsetDateTime`] for convenient.
    pub fn last_modified(&self) -> Option<OffsetDateTime> {
        self.meta.last_modified()
    }

    /// ETag of this object.
    ///
    /// `ETag` is defined by [RFC 7232](https://httpwg.org/specs/rfc7232.html#header.etag)
    /// Refer to [MDN ETag](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag) for more information.
    ///
    /// OpenDAL will return this value AS-IS like the following:
    ///
    /// - `"33a64df551425fcc55e4d42a148795d9f25f89d4"`
    /// - `W/"0815"`
    ///
    /// `"` is part of etag.
    ///
    /// # Notes
    ///
    /// The etag returned here is the etag of this read request.
    /// It's **different** from the object's etag.
    pub fn etag(&self) -> Option<&str> {
        self.meta.etag()
    }
}

impl AsyncRead for ObjectReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}