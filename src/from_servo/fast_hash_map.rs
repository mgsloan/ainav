// FastHashMap from https://github.com/servo/webrender/blob/1ce4ac12f80b015cd88cd71f41cf5c1430528a18/webrender/src/internal_types.rs

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use fxhash::FxHasher;
use std::collections::{HashMap, HashSet};

pub type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
