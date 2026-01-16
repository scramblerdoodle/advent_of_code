package utils

type Queue[T comparable] struct {
	Items []T
}

func (q *Queue[T]) Enqueue(item T) {
	q.Items = append(q.Items, item)
}

func (q *Queue[T]) Dequeue() (T, bool) {
	if len(q.Items) == 0 {
		var zero T
		return zero, false
	}

	item, remaining := q.Items[0], q.Items[1:]
	q.Items = remaining
	return item, true
}

func (q *Queue[T]) Find(target T) (int, bool) {
	for i, v := range q.Items {
		if v == target {
			return i, true
		}
	}

	return -1, false
}
