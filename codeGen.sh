bindgen src/s2wrapper.hpp \
  --allowlist-type "S2LatLng.*"\
  --allowlist-type "S2Point.*"\
  --allowlist-type "S2CellIdIndex.*"\
  --allowlist-type "S2ClosestCellQuery.*"\
  --allowlist-type "S2RegionTermIndexer.*"\
  --allowlist-type "Options.*"\
  --allowlist-type "ctx"\
  --allowlist-function "S2Region.*"\
  --blocklist-type "const_pointer"\
  --opaque-type "absl.*"\
  --opaque-type "std.*"\
  --opaque-type "gtl.*"\
  --no-layout-tests\
  --size_t-is-usize\
  -- -std=c++17 -D_GLIBCXX_USE_CXX11_ABI=1
