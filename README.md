# Samba Active Directory Tools for IIS Ferraris - Pancaldo

## 1. Purpose
This software suite is developed with the aim to **replace the older**, less optimized **Python** codebase used
to manage the internal **Active Directory** domain. Currently, a set of self-contained Python applications
(CLI tools) are used to manage the domain using the **samba-tool command line program**.

## 2. Why is this needed?
IIS Ferraris - Pancaldo is the largest school in the Province of Savona, Liguria, located on the northern
coast of Italy. The large user base requires efficient management of authentication and identification on
the school network, which is handled by a Samba Active Directory Domain Controller. 
Unfortunately, **the Samba built-in scripting tools are ill-suited** for the kinds of operation that we have
to perform when managing the domain, as it is lacking a lot of the good features that our old domain
management suite (based on LDAP), such as batch user management.

## 3. How will this be implemented?
The use of the Java programming language will allow us to create an **LDAP-based interface** with the Active
Directory domain, allowing us to perform operations at a lower, more sophisticated level. Plus, using
Java allows better performance and flexibility in contrast with sticking with a scripting language such as
Python, which even though it had very good developments lately is still, in my opinion, a scripting language.