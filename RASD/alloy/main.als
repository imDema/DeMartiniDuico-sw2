open util/ordering[Time]

sig Time {}

sig Department{
	maxVisitors: Int,
}
sig Visit {
	departments: set Department
}
fact positiveMaxVisitors{
	all d: Department | d.maxVisitors >= 0
}
sig Shop{
	departments: disj some Department,
	queue: disj TicketListNode -> Time,
}
fact departmentsOfShop {
	all d: Department | some s: Shop | d in s.departments

}
sig TicketListNode {
	ticket: disj Ticket,
	next: lone TicketListNode,
}
fun lastNode[node: TicketListNode]: TicketListNode {
	node.next = none
		implies node
		else lastNode[node.next]
}
pred pop[h, h': TicketListNode, t: Ticket] {
	h' = h.next
	t = h.ticket
}



sig Customer{
	tokens: disj set Token,
	visiting: Visit -> Time,
}
sig RegisteredCustomer extends Customer{

}

sig Staff{
}

sig Manager extends Staff{
	managedShops : set Shop,  
}




abstract sig Token{
	associatedVisit: one Visit,
	shop: Shop
}
fact tokenVisitDepartmentAreOfShop {
	all tok: Token | tok.associatedVisit.departments in tok.shop.departments
}
fact tokenVisitNotEmpty {
	all tok: Token | tok.associatedVisit.departments != none
}
sig Booking extends Token{
	timeSlot: one Time,
}
sig Ticket extends Token{
	
}

fact tokenOwnership {
	// Exists owner
	all tok: Token | 
		some c: Customer | tok in c.tokens
}


pred enter[c: Customer, v: Visit, t, t': Time] {
	some b: Booking | {
		b in c.tokens
		b.timeSlot = t'
		b.associatedVisit = v
	}
	or
	some tick: Ticket |{
		tick in c.tokens
		tick.associatedVisit = v
		pop[tick.shop.queue.t, tick.shop.queue.t',tick]
	}
	all d: v.departments | departmentOccupancy[d, t'] =< d.maxVisitors
	c.visiting.t = none
	c.visiting.t' = v
}

pred exit[c: Customer, v: Visit, t, t': Time] {
	c.visiting.t = v
	c.visiting.t' = none
}

pred stay[c: Customer, v: Visit, t, t': Time] {
	c.visiting.t = v
	c.visiting.t' = v
}

fact Trace {
	all c: Customer | c.visiting.first = none
	all t: Time - last | {
		all c: Customer | {
			some v: Visit | 
				enter[c,v,t,t.next] or
				exit[c,v,t,t.next] or
				stay[c,v,t,t.next]
		}
	}
}

fun departmentOccupancy[d: Department, t: Time]: Int {
	#d.~(visiting.t.departments)
}

assert checkOccupancy{
	all t: Time, d: Department | {
		departmentOccupancy[d, t] =< d.maxVisitors
	}
}
// check checkOccupancy
run{} for 5 but exactly 6 Token, exactly 3 Customer, exactly 3 Ticket, exactly 2 Department
