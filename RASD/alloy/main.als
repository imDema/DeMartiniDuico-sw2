open util/ordering[Time]

sig Time {}

sig Shop{
	departments: disj some Department,
	queue: disj WaitingListNode lone -> Time,
}
fact departmentsOfShop {
	all d: Department | some s: Shop | d in s.departments
}

sig Department{
	maxVisitors: Int,
}
fact positiveMaxVisitors{
	all d: Department | d.maxVisitors >= 0
}
fun departmentOccupancy[d: Department, t: Time]: Int {
	#d.~(visiting.t.departments)
}

// Subset of departments visited
sig Visit {
	departments: some Department
}
fact uniqueVisits {
	all v, v': Visit | v != v' => v.departments != v'.departments
}

sig WaitingListNode {
	ticket: disj Ticket,
	next: lone WaitingListNode,
}
fact waitingListNodeSameShop{
	all t: WaitingListNode | 
		t.next != none => t.ticket.shop = t.next.ticket.shop 
}
fact waitingListNodeNoCycles{
	all t : WaitingListNode | 
		t not in t.^next
}

pred popWaitingList[h, h': WaitingListNode, t: Ticket] {
	h' = h.next
	t = h.ticket
}

sig Customer{
	tokens: disj set Token,
	visiting: Visit lone -> Time,
}

abstract sig Token{
	associatedVisit: one Visit,
	shop: Shop
}
fact tokenVisitShopConsistency {
	all tok: Token | tok.associatedVisit.departments in tok.shop.departments
}
fact tokenIsOwned {
	all tok: Token | some c: Customer | tok in c.tokens
}

sig Booking extends Token{
	timeSlot: one Time,
}
sig Ticket extends Token{}

// Entrance checking

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
		popWaitingList[tick.shop.queue.t, tick.shop.queue.t',tick]
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

// Time consistency
fact Trace {
	all c: Customer | c.visiting.first = none
	all t: Time - last | {
		all c: Customer | {
			some v: Visit |
				enter[c,v,t,t.next] or
				exit[c,v,t,t.next] or
				stay[c,v,t,t.next]
			or stay[c, none, t, t.next]
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

// SCENARIOS

pred enterAndExit {
	some t, t', t'': Time, c: Customer, v: Visit | {
		c.visiting.t = none
		c.visiting.t' = v
		c.visiting.t'' = none
		lt[t, t']
		lt[t', t'']
	}
}
pred enterExitTicketBooking {
	some c1,c2: Customer {
		c1 != c2
		c1.tokens & Ticket = none
		c2.tokens & Booking = none
		some t, t', t'': Time, v: Visit {
			c1.visiting.t = none
			c1.visiting.t' = v
			c1.visiting.t'' = none
			lt[t, t']
			lt[t', t'']
		}
		some t, t', t'': Time, v: Visit {
			c2.visiting.t = none
			c2.visiting.t' = v
			c1.visiting.t'' = none
			lt[t, t']
			lt[t', t'']
		}
	}
}
pred show {
	#Shop = 2
	#Department = 3
	#Ticket = 3
	#Booking = 3
	#Customer = 4
	#Visit = 4
}
check allAssertions for 3 but exactly 10 Time

run show for 6
run {enterAndExit} for 8 but exactly 5 Customer, exactly 6 Token, exactly 3 Booking, exactly 3 Department, exactly 2 Shop, exactly 4 Visit

run {enterExitTicketBooking} for 8 but exactly 5 Customer, exactly 6 Token, exactly 3 Booking, exactly 3 Department, exactly 2 Shop, exactly 4 Visit
