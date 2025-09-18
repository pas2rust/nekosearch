# nekosearch

A Rust toolkit for text search, fuzzy matching, and intent detection.  
From minimal, dependency-free matching to full ranking pipelines with TF-IDF, Jaccard, and Levenshtein.

---

## 📋 Feature Checklist

### 🔹 Core (std-only)
- [ ] Simple tokenization (`split_whitespace`)
- [ ] Basic normalization (lowercase, trim)
- [ ] Exact equality comparison
- [ ] Word-by-word comparison (overlap count)
- [ ] Set similarity (basic Jaccard)
- [ ] Character similarity (Hamming, if lengths match)

### 🔹 Normalization & Preprocessing
- [ ] Remove punctuation
- [ ] Remove stopwords (customizable list)
- [ ] Unicode normalization (NFC/NFD)
- [ ] Accent stripping (configurable)
- [ ] Stemming or lemmatization (at least English/Portuguese)

### 🔹 Similarity Metrics
- [ ] Levenshtein distance
- [ ] Damerau–Levenshtein (transpositions)
- [ ] Sørensen–Dice coefficient
- [ ] Cosine similarity (with TF-IDF vectors)
- [ ] Advanced Jaccard (n-grams)

### 🔹 Indexing & Search
- [ ] Basic inverted index (word → docs)
- [ ] Ranking by TF (Term Frequency)
- [ ] Ranking by TF-IDF
- [ ] Approximate search (configurable threshold)
- [ ] Typo-tolerant search

### 🔹 Fuzzy Matching
- [ ] N-grams (2-gram, 3-gram, etc.)
- [ ] Fast Levenshtein approximation
- [ ] Fuzzy ranking (normalized score 0–1)
- [ ] Partial matching (relevant substrings)

### 🔹 Advanced Features
- [ ] Compound queries (AND, OR, NOT)
- [ ] Custom weighting support
- [ ] Query expansion (synonyms, related terms)
- [ ] Intent detection pipeline
- [ ] Parallel indexing/search (optional `rayon` feature)
- [ ] Serialization of index (optional `serde` feature)
