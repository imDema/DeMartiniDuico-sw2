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
	waitingVisitors: set Customer,
}
fact departmentsOfShop {
	all d: Department | some s: Shop | d in s.departments
}



sig Customer{
	tokens: disj Token lone -> lone Time,
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
	//isRegistered:
	associatedVisit: one Visit,
}
fact visitHasDepartmentsOfOneAndOnlyShop{
	all tok: Token | 
		all d1,d2 : Department | d1+d1 in tok.associatedVisit.departments => some s : Shop | d1 in s.departments and d2 in s.departments
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
		some c: Customer | tok in c.tokens.Time
	
	// Unique owner
	all disj c1, c2: Customer |
			c1.tokens.Time & c2.tokens.Time = none
}






pred enter[c: Customer, v: Visit, t, t': Time] {
	some b: Booking | {
		b in c.tokens.t
		b.timeSlot = t'
		b.associatedVisit = v
	}
	or
	some tick: Ticket |{
		//ticket rules
		tick != none
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
run{} for 5 but exactly 6 Token, exactly 2 Department
