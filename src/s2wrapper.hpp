#include "s2/s2point_index.h"
#include "s2/s2latlng.h"
#include "s2/s2closest_cell_query.h"
#include "s2/s2cell_index.h"
#include "s2/s2region_term_indexer.h"
#include "s2/s2earth.h"
//#include "s2/s2testing.h"
//#include <cstdlib>

using namespace std;
#ifdef __cplusplus
extern "C" {
#endif

void wrapper_test();

std::vector<int> wrapper_test2();

S2ClosestCellQuery::Options wrapper_init_S2ClosestCellQuery_Options();

absl::string_view wrapper_string_view_init(const char *str);

class ctx {
public:
    S2RegionTermIndexer::Options options;
    S2RegionTermIndexer indexer;
    std::vector <S2Point> *documents;

    ctx();

    void load(float const *_points, size_t len);

    void *search(float _lng, float _lat, float radius, size_t *len);

    void clear();
};

#ifdef __cplusplus
}
#endif