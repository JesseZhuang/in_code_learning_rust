// lc 901 - Online Stock Span

/// Monotonic stack approach.
/// Stack stores (price, span) pairs in decreasing order of price.
struct StockSpanner {
    stack: Vec<(i32, i32)>, // (price, span)
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner { stack: Vec::new() }
    }

    // Time: O(n) amortized — each element pushed/popped at most once.
    // Space: O(n) worst case for the stack.
    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some(&(top_price, top_span)) = self.stack.last() {
            if top_price <= price {
                span += top_span;
                self.stack.pop();
            } else {
                break;
            }
        }
        self.stack.push((price, span));
        span
    }
}

/// DP / jump-back approach.
/// Stores all prices and spans; jumps backwards through spans to skip already-counted days.
struct StockSpannerDP {
    prices: Vec<i32>,
    spans: Vec<i32>,
}

impl StockSpannerDP {
    fn new() -> Self {
        StockSpannerDP {
            prices: Vec::new(),
            spans: Vec::new(),
        }
    }

    // Time: O(n) amortized — jumps skip previously counted elements.
    // Space: O(n) for price and span vectors.
    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        let mut i = self.prices.len() as i32 - 1;
        while i >= 0 && self.prices[i as usize] <= price {
            span += self.spans[i as usize];
            i -= self.spans[i as usize]; // jump back by the span at position i
        }
        self.prices.push(price);
        self.spans.push(span);
        span
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_stock_spanner(prices: &[i32]) -> Vec<i32> {
        let mut ss = StockSpanner::new();
        prices.iter().map(|&p| ss.next(p)).collect()
    }

    fn run_stock_spanner_dp(prices: &[i32]) -> Vec<i32> {
        let mut ss = StockSpannerDP::new();
        prices.iter().map(|&p| ss.next(p)).collect()
    }

    #[test]
    fn test_example1() {
        let prices = [100, 80, 60, 70, 60, 75, 85];
        let expected = vec![1, 1, 1, 2, 1, 4, 6];
        assert_eq!(run_stock_spanner(&prices), expected);
        assert_eq!(run_stock_spanner_dp(&prices), expected);
    }

    #[test]
    fn test_all_increasing() {
        let prices = [1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(run_stock_spanner(&prices), expected);
        assert_eq!(run_stock_spanner_dp(&prices), expected);
    }

    #[test]
    fn test_all_decreasing() {
        let prices = [5, 4, 3, 2, 1];
        let expected = vec![1, 1, 1, 1, 1];
        assert_eq!(run_stock_spanner(&prices), expected);
        assert_eq!(run_stock_spanner_dp(&prices), expected);
    }

    #[test]
    fn test_all_same() {
        let prices = [7, 7, 7, 7, 7];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(run_stock_spanner(&prices), expected);
        assert_eq!(run_stock_spanner_dp(&prices), expected);
    }

    #[test]
    fn test_valley_and_peak() {
        // Descend then ascend: valley at 1, peak at 6 spans everything
        let prices = [5, 3, 1, 2, 4, 6];
        let expected = vec![1, 1, 1, 2, 4, 6];
        assert_eq!(run_stock_spanner(&prices), expected);
        assert_eq!(run_stock_spanner_dp(&prices), expected);
    }

    #[test]
    fn test_alternating() {
        let prices = [2, 5, 2, 5, 2, 5];
        let expected = vec![1, 2, 1, 4, 1, 6];
        assert_eq!(run_stock_spanner(&prices), expected);
        assert_eq!(run_stock_spanner_dp(&prices), expected);
    }

    #[test]
    fn test_boundary_values() {
        // Single element
        assert_eq!(run_stock_spanner(&[1]), vec![1]);
        assert_eq!(run_stock_spanner_dp(&[1]), vec![1]);

        // Large price value
        let prices = [100000, 1, 100000];
        let expected = vec![1, 1, 3];
        assert_eq!(run_stock_spanner(&prices), expected);
        assert_eq!(run_stock_spanner_dp(&prices), expected);
    }
}
