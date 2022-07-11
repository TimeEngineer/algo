### Structure

```
Operations which have an amortized cost are suffixed with a *.
Operations with an expected cost are suffixed with a ~.
```

## Collections

| Name       | get(i)         | insert(i)       | append |
|------------|----------------|-----------------|--------|
| Vec        | O(1)           | O(n-i)*         | O(m)*  |
| VecDeque   | O(1)           | O(min(i, n-i))* | O(m)*  |
| LinkedList | O(min(i, n-i)) | O(min(i, n-i))  | O(1)   |
| HashMap    | O(1)~          | O(1)~*          | N/A    |
| BTreeMap   | O(log(n))      | O(log(n))       | O(n+m) |
| BinaryHeap | N/A            | N/A             | O(n+m) |

## Graph

