

rbm-t.TestRBM.mk:
	@echo "Show that we can access the example"
	podman exec -it xtsnp-bottle nslookup example.com
	@echo "Show that we cannot access google"
	! podman exec -it xtsnp-bottle nslookup google.com
	@echo "PASS"

