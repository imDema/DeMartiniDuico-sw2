open util/ordering[Time]

sig Time {}

sig Department{
	maxVisitors: Int,
}
sig Visit {
	departments: some Department
}
fact positiveMaxVisitors{
	all d: Department | d.maxVisitors >= 0
}
sig Shop{
	departments: disj some Department,
	queue: disj TicketListNode lone -> Time,
}
fact departmentsOfShop {
	all d: Department | some s: Shop | d in s.departments

}
sig TicketListNode {
	ticket: disj Ticket,
	next: lone TicketListNode,
}
fact ticketListNodeSameShop{
	all t: TicketListNode | 
		t.next != none => t.ticket.shop = t.next.ticket.shop 
}
fact ticketListNodeNoCycles{
	all t : TicketListNode | 
		t not in t.^next
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
	visiting: Visit lone -> Time,
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


pred hasValidBooking[c: Customer, v: Visit, t, t': Time] {
	some b: Booking | {
		b in c.tokens
		b.timeSlot = t'
		b.associatedVisit = v
	}
}

pred hasValidTicket[c: Customer, v: Visit, t, t': Time] {
	some tick: Ticket |{
		tick in c.tokens
		tick.associatedVisit = v
		pop[tick.shop.queue.t, tick.shop.queue.t',tick]
		tick.shop.queue.t' not in tick.shop.queue.(prevs[t'])
	}
}

pred enter[c: Customer, v: Visit, t, t': Time] {
	hasValidBooking[c,v,t,t'] or hasValidTicket[c,v,t,t']
	
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
		all s: Shop | {
			s.queue.t = s.queue.(t.next) or {
				some c: Customer, v: Visit | {
					v.departments in s.departments
					hasValidTicket[c,v,t,t.next]
				}
			}
		}
	}
}

fun departmentOccupancy[d: Department, t: Time]: Int {
	#d.~(visiting.t.departments)
}

// ASSERTIONS

assert allAssertions {
	checkOccupancy
	cannotEnterWithoutToken
	cannotEnterAtDifferentTimeWithBooking
	cannotSkipQueue
}

pred checkOccupancy{
	all t: Time, d: Department | {
		departmentOccupancy[d, t] =< d.maxVisitors
	}
}
pred cannotEnterWithoutToken {
	no c: Customer | {
		some t: Time | { 
			c.visiting.t != none
			c.visiting.t not in c.tokens.associatedVisit
		}
	}
}
pred cannotEnterAtDifferentTimeWithBooking {
	no c: Customer, v: Visit, t: Time | {
		// Customer has no tickets
		c.tokens & Ticket = none

		// Customer visits some departments 
		c.visiting.t = none
		c.visiting.(t.next) = v

		no b: Booking | {
			b in c.tokens
			b.associatedVisit = v
			b.timeSlot = t.next
		}
	}
}
pred cannotSkipQueue {
	no c: Customer, v: Visit, t: Time | {
		// Customer has no booking
		c.tokens & Booking = none

		// Customer visits some departments 
		c.visiting.t = none
		c.visiting.(t.next) = v

		no tick: Ticket | {
			tick in c.tokens
			tick.associatedVisit = v
			tick in tick.shop.queue.(prevs[t] + t).ticket // Ticket was in queue
			tick not in tick.shop.queue.(nexts[t]).ticket // Ticket was used
		}
	}
}
pred canEnterAndExit {
	some t, t', t'': Time, c: Customer, v: Visit | {
		c.visiting.t = none
		c.visiting.t' = v
		c.visiting.t'' = none
		lt[t, t']
		lt[t', t'']
	}
}

check allAssertions for 5

run canEnterAndExit for 8 but exactly 5 Customer, exactly 3 Token, exactly 3 Ticket
