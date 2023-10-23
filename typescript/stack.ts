type Node<T>={
    value: T,
    prev?: Node<T>,
}

export default class Stack<T>{
    public length:number;
    private head?:Node<T>;

    constructor(){
        this.length=0;
        this.head=undefined;
    }

    push(item:T):void{
        const node={value:item} as Node<T>;
        this.length++;
        if (!this.head){
            this.head=node;
            return;
        }
        const top=this.head;

        //this.head=node;
        //this.head.prev=top;
        
        //prime did method below, I did above
        node.prev=top;
        this.head=node;
    }

    pop():T|undefined{
        this.length=Math.max(0,this.length-1);
        if (this.length===0){
            const head =this.head;
            this.head=undefined;
            return head?.value;
        }
        const head =this.head as Node<T>;
        const previous=head.prev;
        //this.head.prev=undefined;
        this.head=previous;
        return head.value;


        //my method commented out below, similar to queue implementation
        //if (!this.head){
        //    return undefined;
        //}
        //this.length--;
        //const top=this.head;
        //this.head=top.prev;
        //top.prev=undefined;
        //return top.value;
    }

    peek():T|undefined{
        return this.head?.value;
    }
}

