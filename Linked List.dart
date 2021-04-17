
import 'dart:io';

class Node{
  int value = 0;
  Node? next;
  Node(int val){
    this.value = val;
    this.next = null;
  }
}

class LinkedList{
  Node? _root;
  LinkedList(){
    this._root = null;
  }
  push(int val){
    Node newNode = new Node(val);
    if(this._root == null){
      this._root = newNode;
    }else{
      Node? temp = _root;
      while(temp?.next != null){
        temp = temp?.next;
      }
      temp?.next = newNode;
    }
  }
  display(){
    Node? temp = _root;
    while(temp != null) {
      stdout.write("${temp.value}->");
      temp = temp.next;
    }
  }
}


void main(){

  LinkedList? ll = new LinkedList();

  ll.push(1);
  ll.push(2);
  ll.push(3);
  ll.push(4);
  ll.push(5);

  ll.display();

}
