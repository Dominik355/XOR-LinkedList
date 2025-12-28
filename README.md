rust import of xor linkedlist idea I saw in tsoding's video: https://gist.github.com/rexim/6f2349b548fdead7ed790d1a40915ae1#file-ll-c

Implementation allocates 2/3 of a std'LinkedList memory

```
 Iters: 1000
XorLinkedList: 15.625 KiB
StdLinkedList: 23.4375 KiB

 Iters: 10000
XorLinkedList: 156.25 KiB
StdLinkedList: 234.375 KiB

 Iters: 100000
XorLinkedList: 1.52587890625 MiB
StdLinkedList: 2.288818359375 MiB

 Iters: 1000000
XorLinkedList: 15.2587890625 MiB
StdLinkedList: 22.88818359375 MiB

 Iters: 10000000
XorLinkedList: 152.587890625 MiB
StdLinkedList: 228.8818359375 MiB
```