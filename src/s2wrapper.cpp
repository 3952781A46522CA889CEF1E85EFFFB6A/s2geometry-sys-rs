#include "s2wrapper.hpp"

ctx::ctx() {
    S2RegionTermIndexer::Options options;
    options.set_index_contains_points_only(true);
    this->options = options;
    S2RegionTermIndexer indexer(options);
    this->indexer = std::move(indexer);
    std::vector <S2Point> *documents = new std::vector<S2Point>;
    this->documents = documents;
    std::unordered_map <string, std::vector<int>> *index = new std::unordered_map <string, std::vector<int>>;
    this->index = index;
}

void ctx::load(float const *_points, size_t len) {
    documents->clear();
    index->clear();
    for (int i = 0; i < len; i += 2) {
        S2LatLng _tmp = S2LatLng::FromDegrees(*(_points + i + 1), *(_points + i));
        documents->push_back(_tmp.ToPoint());
    }
    for (int docid = 0; docid < documents->size(); ++docid) {
        S2Point index_region = (*documents)[docid];
        for (const auto &term: indexer.GetIndexTerms(index_region, kPrefix)) {
            (*index)[term].push_back(docid);
        }
    }
}

void *ctx::search(float _lng, float _lat, float _radius, size_t *_len) {
    S1Angle radius = S1Angle::Radians(S2Earth::KmToRadians(_radius));
    S2LatLng tmp = S2LatLng::FromDegrees(_lat, _lng);
    S2Point center = tmp.ToPoint();
    S2Cap query_region(center, radius);
    std::set<int> candidates;

    for (const auto &term: indexer.GetQueryTerms(query_region, kPrefix)) {
        candidates.insert((*index)[term].begin(), (*index)[term].end());
    }

    vector<int> *result = new vector<int>;

    for (int docid: candidates) {
        if (!query_region.Contains((*documents)[docid])) continue;
        (*result).push_back(docid);
    }

    *_len = (*result).size();
    return &((*result)[0]);
}

void ctx::clear() {
    documents->clear();
    index->clear();
}

void wrapper_test() {
    S2PointIndex<int> index;
    S2Point point(S2LatLng::FromDegrees(30.2, 112.0));
    index.Add(point, 3);
    std::cout << "wrapper_test pass" << std::endl;
}

std::vector<int> wrapper_test2() {
    // S2ClosestCellQuery_options();
    // S2ClosestCellQuery_Options::from()
    static const char kPrefix[] = "s2:";
    std::vector <S2Point> documents;
    S1Angle radius = S1Angle::Radians(S2Earth::KmToRadians(0.5));

    S2LatLng a = S2LatLng::FromDegrees(31.2, 112.3);
    S2LatLng b = S2LatLng::FromDegrees(31.2, 113.4);
    S2LatLng c = S2LatLng::FromDegrees(31.2, 114.5);

    documents.push_back(a.ToPoint());
    documents.push_back(b.ToPoint());
    documents.push_back(c.ToPoint());

    S2RegionTermIndexer::Options options;
    S2RegionTermIndexer indexer(options);
    S2Point center = a.ToPoint();
    S2Cap query_region(center, radius);
    std::set<int> candidates;
    std::unordered_map <string, std::vector<int>> index;

    for (int docid = 0; docid < documents.size(); ++docid) {
        S2Point index_region = documents[docid];
        for (const auto &term: indexer.GetIndexTerms(index_region, kPrefix)) {
            index[term].push_back(docid);
        }
    }

    for (const auto &term: indexer.GetQueryTerms(query_region, kPrefix)) {
        candidates.insert(index[term].begin(), index[term].end());
    }

    std::vector<int> result;
    for (int docid: candidates) {
        if (!query_region.Contains(documents[docid])) continue;
        result.push_back(docid);
    }
    return result;
}

S2ClosestCellQuery::Options wrapper_init_S2ClosestCellQuery_Options() {
    S2ClosestCellQuery::Options index;
    return index;
}

absl::string_view wrapper_string_view_init(const char *str) {
    absl::string_view a = absl::string_view(str);
    return a;
}
