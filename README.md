# Software Engineering 2 Project
**De Martini Luca, Duico Alessandro**
## Presentation of the project
For an overview of the project we suggest to take a look at the [slideshow](/DeliveryFolder/slides.pdf).
### Purpose
During the COVID-19 pandemic, governments have introduced various measures aimed at reducing to a minimum close contacts between people. These measures include social distancing and lockdowns. Access to essential activities, such as supermarkets, should be limited to lower the density of people inside the activities and allow keeping distances effectively and mitigate the risk.
<br>
This application attempts at mitigating the problem of forming lines by digitally *handing a "number"* to the clients. This way people will only need to go to the shop when it is their turn to enter and they will not need to hang around the building.
<br>
The product also allows store managers to effectively monitor entrances by scanning a QR code associated with the “number”, to ensure that the safety limits are met.
In addition to the queueing mechanism, CLup will also allow customers to choose the approximate time of their visit and the category of items they wish to purchase, allowing for more accurate forecasting of wait times and knowing the areas they will be visiting, to make better use of space in the building.

### Architecture
Clup is a three-tier system, consisting of a **Database** (Postgres), the **Clup binary** (ITD/backend) and the **Clup Webapp** (ITD/frontend). A middleware (Redis) is employed for session storage.


## Deliverables
### Documents
[Requirement Analysis and Specification Document (RASD)](/DeliveryFolder/RASD1.pdf)

[Design Document (DD)](/DeliveryFolder/DD1.1.pdf)

[Implementation and Testing Document (ITD)](/DeliveryFolder/ITD1.1.pdf)

### Artifacts

[DeliveryFolder/ITD/artifacts](/DeliveryFolder/ITD/artifacts/)

Execute `run.sh` to deploy the pre-built Docker containers automatically.

For more information on deployment, please read the documentation in [DeliveryFolder/ITD/backend-docs.zip](/DeliveryFolder/ITD/backend-docs.zip)
