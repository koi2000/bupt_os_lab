use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ptr::{self, null_mut};

type Link<T> = *mut Node<T>;
/// 双链表
pub struct LinkedList<T> {
    // TODO: YOUR CODE HERE
    head: Link<T>,
    tail: Link<T>,
    length: i32,
    // marker: PhantomData<T>, // 可以去掉
}

/// 链表节点
struct Node<T> {
    // TODO: YOUR CODE HERE
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

/// 链表迭代器
pub struct Iter<'a, T> {
    // TODO: YOUR CODE HERE
    // marker: PhantomData<&'a T>,
    next: Option<&'a Node<T>>,
    prev: Option<&'a Node<T>>,
}

/// 链表可变迭代器
pub struct IterMut<'a, T> {
    // TODO: YOUR CODE HERE
    // marker: PhantomData<&'a mut T>,
    next: Option<&'a mut Node<T>>,
    prev: Option<&'a mut Node<T>>,
}

impl<T> LinkedList<T> {
    /// 创建一个空链表
    pub fn new() -> Self {
        // Self {
        //     // TODO: YOUR CODE HERE
        //     marker: PhantomData,
        // }
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            length: 0,
        }
        // unimplemented!()
    }

    /// 将元素插入到链表头部
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1);
    /// assert_eq!(list.front(), Some(&1));
    /// ```
    pub fn push_front(&mut self, _elem: T) {
        // TODO: YOUR CODE HERE
        // unimplemented!()
        self.length += 1;
        unsafe {
            let new_head = Box::into_raw(Box::new(Node {
                elem: _elem,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            if !self.head.is_null() {
                (*new_head).next = self.head;
                (*self.head).prev = new_head;
            } else {
                self.tail = new_head;
            }
            self.head = new_head;
        }
    }

    /// 将元素插入到链表尾部
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.back(), Some(&1));
    /// ```
    pub fn push_back(&mut self, _elem: T) {
        // TODO: YOUR CODE HERE
        self.length += 1;
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem: _elem,
                next: ptr::null_mut(),
                prev: self.tail,
            }));
            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }
            self.tail = new_tail;
        }
        // unimplemented!()
    }

    /// 将第一个元素返回
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1);
    /// assert_eq!(list.pop_front(), Some(1));
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        // TODO: YOUR CODE HERE
        // unimplemented!()
        self.length -= 1;
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                } else {
                    // 将头更新为空
                    (*self.head).prev = ptr::null_mut();
                }
                Some(head.elem)
            }
        }
    }

    /// 将最后一个元素返回
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.pop_back(), Some(1));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        // TODO: YOUR CODE HERE
        // unimplemented!()
        self.length -= 1;
        unsafe {
            if self.tail.is_null() {
                None
            } else {
                let tail = Box::from_raw(self.tail);
                self.tail = tail.prev;

                if self.tail.is_null() {
                    self.head = ptr::null_mut();
                } else {
                    // 将头更新为空
                    (*self.tail).next = ptr::null_mut();
                }
                Some(tail.elem)
            }
        }
    }

    /// 返回链表第一个元素的引用  
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.front(), None);
    /// list.push_front(1);
    /// assert_eq!(list.front(), Some(&1));
    /// ```
    pub fn front(&self) -> Option<&T> {
        // TODO: YOUR CODE HERE
        // unimplemented!()
        // Some(&*self.head)
        unsafe {
            if self.head.is_null() {
                None
            } else {
                Some(&(*self.head).elem)
            }
        }
    }

    /// 返回链表第一个元素的可变引用   
    pub fn front_mut(&mut self) -> Option<&mut T> {
        // TODO: YOUR CODE HERE
        // unimplemented!()
        unsafe {
            if self.head.is_null() {
                None
            } else {
                Some(&mut (*self.head).elem)
            }
        }
    }

    /// 返回链表最后一个元素的引用
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.back(), None);
    /// list.push_back(1);
    /// assert_eq!(list.back(), Some(&1));
    /// ```
    pub fn back(&self) -> Option<&T> {
        // TODO: YOUR CODE HERE
        // unimplemented!()
        unsafe {
            if self.tail.is_null() {
                None
            } else {
                Some(&(*self.tail).elem)
            }
        }
    }

    /// 返回链表最后一个元素的可变引用
    pub fn back_mut(&mut self) -> Option<&mut T> {
        // TODO: YOUR CODE HERE
        // unimplemented!()
        unsafe {
            if self.tail.is_null() {
                None
            } else {
                Some(&mut (*self.tail).elem)
            }
        }
    }

    /// 返回链表长度
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        // TODO: YOUR CODE HERE
        // unimplemented!()
        self.length as usize
    }

    /// 判断链表是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 清空链表
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.len(), 2);
    /// list.clear();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        // Oh look it's drop again
        while self.pop_front().is_some() {}
    }

    /// 返回一个迭代器
    pub fn iter(&self) -> Iter<T> {
        // Iter {
        //     // TODO: YOUR CODE HERE
        //     marker : PhantomData,
        // }
        // unimplemented!();
        unsafe {
            Iter {
                next: self.head.as_ref(),
                prev: self.tail.as_ref(),
            }
        }
    }

    /// 返回一个可变迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        // IterMut {
        //     // TODO: YOUR CODE HERE
        //     marker: PhantomData,
        // }
        // unimplemented!();
        unsafe {
            IterMut {
                next: self.head.as_mut(),
                prev: self.tail.as_mut(),
            }
        }
    }

    /// 获取链表中指定位置的元素   
    /// 如果超出范围，使用panic!宏抛出异常
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.get(0), &1);
    /// ```
    pub fn get(&self, _at: usize) -> &T {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        unsafe {
            if _at >= self.len() {
                panic!("out of bounds");
            }
            for (i, j) in self.iter().enumerate() {
                if i == _at {
                    return j;
                }
            }
            &(*self.head).elem
        }
    }

    /// 获取链表中指定位置的可变元素
    pub fn get_mut(&mut self, _at: usize) -> &mut T {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        unsafe {
            let mut current = self.head.as_mut();
            for _ in 0.._at {
                match current {
                    Some(node) => {
                        current = node.next.as_mut();
                    }
                    None => panic!("Out of bounds"),
                }
            }
            match current {
                Some(node) => &mut node.elem,
                None => panic!("Out of bounds"),
            }
        }
    }

    /// 将元素插入到**下标为i**的位置    
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.insert(0,1);
    /// list.insert(1,3);
    /// list.insert(1,2);
    /// assert_eq!(list.get(0), &1);
    /// assert_eq!(list.get(1), &2);
    /// assert_eq!(list.get(2), &3);
    /// ```
    pub fn insert(&mut self, _at: usize, _data: T) {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        unsafe {
            self.length += 1;
            if _at > self.len() {
                panic!("out of bounds");
            }
            let mut current = self.head.as_mut();
            for _ in 0.._at - 1 {
                match current {
                    Some(node) => {
                        current = node.next.as_mut();
                    }
                    None => panic!("Out of bounds"),
                }
            }
            let new_node = Box::into_raw(Box::new(Node {
                elem: _data,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            match current {
                Some(node) => {
                    // 新节点指向旧节点的下一个
                    (*new_node).next = node.next;
                    // 旧节点的下一个的prev指向新节点
                    if !node.next.is_null() {
                        (*node.next).prev = new_node;
                    }

                    (*node).next = new_node;
                    (*new_node).prev = node;
                }
                None => {
                    self.head = new_node;
                    self.tail = new_node;
                }
            }
        }
    }

    /// 移除链表中下标为i的元素
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3]);
    /// assert_eq!(list.remove(1), 2);
    pub fn remove(&mut self, _at: usize) -> T {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        if _at >= self.len() {
            panic!("Out of bounds");
        }
        self.length -= 1;
        let mut current = self.head;
        let mut prev: *mut Node<T> = std::ptr::null_mut();

        for i in 0.._at {
            unsafe {
                match current.as_mut() {
                    Some(node) => {
                        prev = node;
                        current = (*node).next;
                    }
                    None => panic!("Out of bounds"),
                }
            }
        }

        unsafe {
            let removed_node = if prev.is_null() {
                let node = self.head;
                self.head = (*node).next;
                Box::from_raw(node)
            } else {
                let node = (*prev).next;
                (*prev).next = (*node).next;
                Box::from_raw(node)
            };

            let elem = std::ptr::read(&removed_node.elem);
            std::mem::forget(removed_node);

            elem
        }
    }

    /// 将链表分割成两个链表，原链表为[0,at-1]，新链表为[at,len-1]。
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3,4,5]);
    /// let new = list.split_off(2); // list = [1,2], new = [3,4,5]
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    pub fn split_off(&mut self, _at: usize) -> LinkedList<T> {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        unsafe {
            if _at > self.len() {
                panic!("Out of bounds");
            }

            let mut new_list = LinkedList {
                head: std::ptr::null_mut(),
                tail: std::ptr::null_mut(),
                length: 0,
            };

            if _at == 0 {
                std::mem::swap(self, &mut new_list);
            } else {
                let mut current = &mut self.head;

                for _ in 0..(_at) {
                    current = &mut (*(*current)).next;
                }

                new_list.head = *current;
                new_list.tail = self.tail;
                new_list.length = self.length - _at as i32;

                self.length = _at as i32;
                self.tail = (*(*current)).prev;
                (*self.tail).next = std::ptr::null_mut();
            }
            new_list
        }
    }

    /// 查找链表中第一个满足条件的元素
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3]);
    /// assert_eq!(list.find_mut(|x| x % 2 == 0), Some(&mut 2));
    /// assert_eq!(list.find_mut(|x| x % 4 == 0), None);
    /// ```
    pub fn find_mut<P>(&mut self, predicate: P) -> Option<&mut T>
    where
        P: Fn(&T) -> bool,
    {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        for (i, j) in self.iter_mut().enumerate() {
            if predicate(j) {
                return Some(j);
            }
        }
        None
    }
}

impl<T: PartialEq> LinkedList<T> {
    /// 判断链表中是否包含某个元素
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.contains(&1), true);
    /// assert_eq!(list.contains(&2), false);
    /// ```
    pub fn contains(&mut self, _data: &T) -> bool {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        for (i, j) in self.iter().enumerate() {
            if j == _data {
                return true;
            }
        }
        false
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // 返回下一个元素，当没有元素可返回时，返回None
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }

    // 返回(self.len, Some(self.len))即可
    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO: YOUR CODE HERE
        unimplemented!();
        // (self.length as usize,Some(self.length as usize))
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                // self.prev = Some(node);
                &mut node.elem
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}

impl<'a, T: 'a> DoubleEndedIterator for Iter<'a, T> {
    // 返回前一个元素
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        unsafe {
            self.prev.map(|node| {
                self.prev = node.prev.as_ref();
                // self.next = Some(node);
                &node.elem
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        // unimplemented!();
        unsafe {
            self.prev.take().map(|node| {
                self.prev = node.prev.as_mut();
                // self.next = Some(node);
                &mut node.elem
            })
        }
    }
}

/// 提供归并排序的功能
pub trait MergeSort {
    /// 就地进行归并排序，按从小到大排序
    fn merge_sort(&mut self);
}

impl<T: PartialOrd + Default> LinkedList<T> {
    // 你可以在这里添加你需要的辅助函数
}

impl<T: PartialOrd + Default> MergeSort for LinkedList<T> {
    fn merge_sort(&mut self) {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Pop until we have to stop
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}
impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}
